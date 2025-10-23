use macroquad::prelude::*;
use macroquad::audio::{Sound, load_sound_from_bytes, play_sound, PlaySoundParams};
use std::fs;
use std::path::Path;
// base64 imports (use below)
// Windows Beep removed (unused)

// Minimal LCG
struct Lcg {
    state: u64,
}
impl Lcg {
    fn new(seed: u64) -> Self { Self { state: seed } }
    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345) & 0x7fffffff;
        self.state
    }
    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        let r = self.next();
        lo + (r % ((hi - lo + 1) as u64)) as i32
    }
}

struct FloatingText {
    x: f32,
    y: f32,
    text: String,
    life: f32,
    size: f32,
}
impl FloatingText {
    fn new(x: f32, y: f32, text: String) -> Self {
        Self { x, y, text, life: 1.0, size: 24.0 }
    }
    fn update(&mut self, dt: f32) {
        self.y -= 30.0 * dt;
        self.life -= dt * 0.6;
        self.size += dt * 4.0;
    }
}

struct Zombie {
    x: f32,
    y: f32,
    hp: f32,
    speed: f32,
    shake: f32,
    elite: bool,
}

// pickups removed: buffs are now applied immediately on drop

// helper: generate a short gunshot WAV in memory and load as Sound
async fn make_shot_sound() -> Sound {
    // 44100 Hz, mono, 16-bit PCM, short decay sine + noise
    let sample_rate = 44100u32;
    let dur_secs = 0.14f32;
    let n_samples = (sample_rate as f32 * dur_secs) as usize;
    let freq = 900.0f32;
    let mut samples: Vec<i16> = Vec::with_capacity(n_samples);
    for i in 0..n_samples {
        let t = i as f32 / sample_rate as f32;
        // decaying sine + small noise
        let env = (-6.0 * t).exp();
        let s = ( (t*freq*2.0*std::f32::consts::PI).sin() * env * 0.9 ) as f32;
        let noise = (rand::gen_range(-0.05f32, 0.05f32) * env) as f32;
        let v = ((s + noise) * (i16::MAX as f32 * 0.45)) as i16;
        samples.push(v);
    }
    // build WAV bytes (RIFF PCM 16-bit)
    let mut wav: Vec<u8> = Vec::new();
    let datasize = (samples.len() * 2) as u32;
    // RIFF header
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36u32 + datasize).to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    // fmt subchunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes()); // subchunk1 size
    wav.extend_from_slice(&1u16.to_le_bytes()); // PCM
    wav.extend_from_slice(&1u16.to_le_bytes()); // channels
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    let byte_rate = sample_rate * 2;
    wav.extend_from_slice(&byte_rate.to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes()); // block align
    wav.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
    // data subchunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&datasize.to_le_bytes());
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    // load into macroquad Sound
    load_sound_from_bytes(&wav).await.unwrap()
}

use base64::engine::general_purpose;
use base64::Engine as _;

fn ensure_assets() {
    // small 1x1 PNG samples (base64) - will be scaled when drawn
    const PLAYER_PNG_B64: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR4nGNgYAAAAAMAASsJTYQAAAAASUVORK5CYII="; // white
    const ZOMBIE_PNG_B64: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR4nGNgYAAAAAMAASsJTYQAAAAASUVORK5CYII="; // white
    let assets_dir = Path::new("assets");
    if !assets_dir.exists() {
        let _ = fs::create_dir_all(assets_dir);
    }
    let p_player = assets_dir.join("player.png");
    let p_zombie = assets_dir.join("zombie.png");
    if !p_player.exists() {
        if let Ok(bytes) = general_purpose::STANDARD.decode(PLAYER_PNG_B64) {
            let _ = fs::write(p_player, bytes);
        }
    }
    if !p_zombie.exists() {
        if let Ok(bytes) = general_purpose::STANDARD.decode(ZOMBIE_PNG_B64) {
            let _ = fs::write(p_zombie, bytes);
        }
    }
}

struct Bullet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}


#[macroquad::main("Zombie Shooter")]
async fn main() {
    // ensure assets exist (write sample PNGs if missing)
    ensure_assets();
    // outer loop allows restarting the game (Try Again)
    loop {
        let mut rng = Lcg::new(123456789);
        // try loading textures (assets/player.png, assets/zombie.png)
        let player_tex = load_texture("assets/player.png").await.ok();
    let zombie_tex = load_texture("assets/zombie.png").await.ok();
    let elite_zombie_tex = load_texture("assets/zombie2.png").await.ok();
        let max_health: i32 = 100;
        let mut player_health: i32 = max_health;
        // unlimited bullets
    let mut exp: i32 = 0;
    let mut round: i32 = 1;
    let mut level: i32 = 1;
    let _zombie_health: i32 = rng.range(2, 4);
    let mut floating_texts: Vec<FloatingText> = Vec::new();
    // player buff state
    let mut player_damage: f32 = 1.0; // base damage (float)
    let mut damage_boost_amount: f32 = 0.0;
    let mut damage_boost_timer: f32 = 0.0;
    let mut global_bleed_dps: f32 = 0.0;
    let mut global_bleed_timer: f32 = 0.0;
    let mut shotgun_extra: i32 = 0;
    let mut shotgun_timer: f32 = 0.0;
    // auto-shoot state
    let mut auto_shoot: bool = false;
    let mut auto_shot_timer: f32 = 0.0;
    let auto_shot_interval: f32 = 0.25; // seconds between auto shots
    let mut muzzle_flash_time: f32 = 0.0;
    let mut last_shot_dx: f32 = 0.0;
    let mut last_shot_dy: f32 = -1.0;
    let mut paused: bool = false;
    // level up animation
    let mut level_up_anim: f32 = 0.0;

        // entities
    let mut zombies: Vec<Zombie> = Vec::new();
        let mut bullets: Vec<Bullet> = Vec::new();
    let mut spawn_timer: f32 = 0.0;
        let spawn_interval: f32 = 0.8; // seconds
    // difficulty scaling
    let mut difficulty_timer: f32 = 0.0;
    let mut difficulty_multiplier: f32 = 1.0; // multiplies base hp
    // round / elite chance HUD
    let mut elite_chance: i32 = 20; // base 20% chance for elite

    // red line y coordinate and player position (moved down)
    let red_line_y = screen_height() * 0.75;
    let player_x = screen_width() / 2.0;
    let player_y = red_line_y + 80.0;

    // prepare gunshot sound (async load)
    let shot_sound = make_shot_sound().await;

        loop {
            clear_background(LIGHTGRAY);
            let dt = get_frame_time();

            // HUD top-left (Round / Level / Exp)
            draw_text(&format!("Round: {}  Level: {}  Exp: {}", round, level, exp), 20.0, 30.0, 26.0, BLACK);

            // Health bar (top-left)
            let hb_x = 20.0;
            let hb_y = 50.0;
            let hb_w = 220.0;
            let hb_h = 22.0;
            let health_frac = (player_health.max(0) as f32) / (max_health as f32);
            draw_rectangle(hb_x - 2.0, hb_y - 2.0, hb_w + 4.0, hb_h + 4.0, DARKGRAY);
            draw_rectangle(hb_x, hb_y, hb_w, hb_h, RED);
            draw_rectangle(hb_x, hb_y, hb_w * health_frac, hb_h, GREEN);
            draw_text(&format!("{} / {}", player_health.max(0), max_health), hb_x + hb_w + 10.0, hb_y + hb_h - 2.0, 20.0, BLACK);

            // Ammo display removed (infinite ammo)

            // show active buffs and damage
            let mut bh = 160.0;
            draw_text(&format!("Damage: {:.1}", player_damage + damage_boost_amount), 20.0, bh, 20.0, ORANGE); bh += 22.0;
            if damage_boost_amount > 0.0 { draw_text(&format!("Damage +{:.1} ({:.0}s)", damage_boost_amount, damage_boost_timer), 20.0, bh, 20.0, ORANGE); bh += 22.0 }
            if global_bleed_dps > 0.0 { draw_text(&format!("Bleed {:.1}/s ({:.0}s)", global_bleed_dps, global_bleed_timer), 20.0, bh, 20.0, RED); bh += 22.0 }
            if shotgun_extra > 0 { draw_text(&format!("Shotgun +{} ({:.0}s)", shotgun_extra, shotgun_timer), 20.0, bh, 20.0, YELLOW); bh += 22.0 }

            // Status (top-right)
            // Pause button (click or press P to toggle)
            let pbtn_w = 120.0;
            let pbtn_h = 28.0;
            let pbtn_x = screen_width() - pbtn_w - 20.0;
            let pbtn_y = 20.0;
            draw_rectangle(pbtn_x, pbtn_y, pbtn_w, pbtn_h, if paused { DARKGRAY } else { LIGHTGRAY });
            draw_text(if paused { "Paused (P)" } else { "Pause (P)" }, pbtn_x + 10.0, pbtn_y + 20.0, 18.0, WHITE);
            // auto-shoot toggle button (below pause button)
            let btn_w = 160.0;
            let btn_h = 28.0;
            let btn_x = screen_width() - btn_w - 20.0;
            let btn_y = pbtn_y + pbtn_h + 8.0;
            draw_rectangle(btn_x, btn_y, btn_w, btn_h, if auto_shoot { DARKGRAY } else { LIGHTGRAY });
            draw_text(if auto_shoot { "Auto-Shoot: ON" } else { "Auto-Shoot: OFF" }, btn_x + 10.0, btn_y + 20.0, 18.0, WHITE);
            // skill button (right-middle)
            let skill_w = 100.0;
            let skill_h = 36.0;
            let skill_x = screen_width() - skill_w - 20.0;
            let skill_y = screen_height()/2.0 - skill_h/2.0;
            draw_rectangle(skill_x, skill_y, skill_w, skill_h, LIGHTGRAY);
            draw_text("哈气", skill_x + 18.0, skill_y + 24.0, 22.0, WHITE);
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                // pause button click
                if mx >= pbtn_x && mx <= pbtn_x + pbtn_w && my >= pbtn_y && my <= pbtn_y + pbtn_h {
                    paused = !paused;
                }
                // auto-shoot button click
                if mx >= btn_x && mx <= btn_x + btn_w && my >= btn_y && my <= btn_y + btn_h {
                    auto_shoot = !auto_shoot;
                }
                // skill button click: 哈气
                if mx >= skill_x && mx <= skill_x + skill_w && my >= skill_y && my <= skill_y + skill_h {
                    // compute skill damage: base 5, +20% per level (multiplicative)
                    let base_skill = 5.0f32;
                    let lvl_mul = if level > 1 { 1.2f32.powi(level - 1) } else { 1.0 };
                    let skill_dmg = base_skill * lvl_mul;
                    // apply to all zombies: push to top and deal damage; handle deaths and buffs
                    let mut new_zombies: Vec<Zombie> = Vec::new();
                    for z in zombies.into_iter() {
                        let mut z = z;
                        // push to top
                        z.y = -60.0;
                        // apply damage
                        z.hp -= skill_dmg;
                        floating_texts.push(FloatingText::new(z.x, z.y, format!("-{:.1}", skill_dmg)));
                        if z.hp <= 0.0 {
                            // grant exp
                            if z.elite { exp += 2; } else { exp += 1; }
                            // 25% chance buff
                            let r = rng.range(0, 99) as i32;
                            if r < 25 {
                                let which = rng.range(0, 3) as i32;
                                match which {
                                    0 => {
                                        player_health = (player_health + 12).min(max_health);
                                        floating_texts.push(FloatingText::new(z.x, z.y, String::from("+12")));
                                    }
                                    1 => {
                                        damage_boost_amount += 1.0;
                                        damage_boost_timer = 8.0;
                                    }
                                    2 => {
                                        global_bleed_dps = 2.0;
                                        global_bleed_timer = 6.0;
                                    }
                                    _ => {
                                        shotgun_extra += 2;
                                        shotgun_timer = 6.0;
                                    }
                                }
                            }
                            // check level up after granting exp
                            let new_level = (exp / 30) + 1;
                            if new_level > level {
                                let gained = new_level - level;
                                for _ in 0..gained { player_damage *= 1.2; }
                                level = new_level;
                                level_up_anim = 0.8;
                                floating_texts.push(FloatingText::new(player_x, player_y - 40.0, String::from("Level Up!")));
                            }
                        } else {
                            new_zombies.push(z);
                        }
                    }
                    zombies = new_zombies;
                }
            }

            // draw red line and player
            draw_line(0.0, red_line_y, screen_width(), red_line_y, 4.0, RED);
                if let Some(tex) = player_tex {
                    let _w = tex.width();
                    let _h = tex.height();
                let mut dest_w = 64.0;
                let mut dest_h = 64.0;
                // level up animation scaling
                if level_up_anim > 0.0 {
                    let s = 1.0 + (level_up_anim * 0.5); // scale factor 1.0 -> up to 1.4
                    dest_w *= s;
                    dest_h *= s;
                }
                draw_texture_ex(tex, player_x - dest_w/2.0, player_y - dest_h/2.0, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(dest_w, dest_h)), source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None });
            } else {
                let mut r = 24.0;
                if level_up_anim > 0.0 { r *= 1.0 + (level_up_anim * 0.5); }
                draw_circle(player_x, player_y, r, BLUE);
                draw_text("PLAYER", player_x - 32.0, player_y + 40.0, 20.0, DARKGRAY);
            }

            // muzzle flash
            if muzzle_flash_time > 0.0 {
                // flash length/offset
                let flash_len = 40.0;
                let fx = player_x + last_shot_dx * flash_len;
                let fy = player_y + last_shot_dy * flash_len;
                draw_circle(fx, fy, 16.0 * (muzzle_flash_time / 0.12), YELLOW);
                draw_circle(fx, fy, 10.0 * (muzzle_flash_time / 0.12), ORANGE);
                // decay
                muzzle_flash_time -= dt;
            }

            // pause handling placed before spawn/update so pause stops movement and timers
            if is_key_pressed(KeyCode::P) { paused = !paused }
            // update level-up animation timer
            if level_up_anim > 0.0 { level_up_anim -= dt; if level_up_anim < 0.0 { level_up_anim = 0.0 } }
            if paused {
                draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.5));
                draw_text("Paused", screen_width()/2.0 - 60.0, screen_height()/2.0 - 40.0, 48.0, WHITE);
                draw_text("Press P to resume, T to restart, Q to quit", screen_width()/2.0 - 250.0, screen_height()/2.0 + 0.0, 24.0, GRAY);
                if is_key_pressed(KeyCode::T) { break }
                if is_key_pressed(KeyCode::Q) { return }
                next_frame().await;
                continue;
            }

            // spawn zombies over time
            spawn_timer += dt;
            if spawn_timer >= spawn_interval {
                spawn_timer = 0.0;
                let zx = rng.range(40, (screen_width() as i32 - 40) as i32) as f32;
                let base_zhp = rng.range(1, 3) as f32;
                // decide elite (base chance 20%, increases each round)
                let is_elite = rng.range(0, 99) < elite_chance;
                // fixed slower zombie speed
                let zspeed = 60.0;
                let mut hp = (base_zhp * difficulty_multiplier).round();
                if is_elite { hp = (hp * 2.0).round(); }
                zombies.push(Zombie { x: zx, y: -40.0, hp: hp as f32, speed: zspeed, shake: 0.0, elite: is_elite });
            }

            // update zombies
            for z in zombies.iter_mut() {
                z.y += z.speed * dt;
            }

            // apply global bleed damage to all zombies each frame (so newly spawned zombies are affected)
            if global_bleed_timer > 0.0 && global_bleed_dps > 0.0 {
                for z in zombies.iter_mut() {
                    z.hp -= global_bleed_dps * dt;
                }
            }

            // draw zombies (with per-zombie shake)
            for z in zombies.iter() {
                let mut zx = z.x;
                let zy = z.y;
                if z.shake > 0.0 {
                    zx += rand::gen_range(-4.0f32, 4.0f32);
                }
                if z.elite {
                    if let Some(tex) = elite_zombie_tex {
                        let _w = tex.width();
                        let _h = tex.height();
                        // larger elite size
                        let dest_w = 80.0;
                        let dest_h = 80.0;
                        draw_texture_ex(tex, zx - dest_w/2.0, zy - dest_h/2.0, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(dest_w, dest_h)), source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None });
                        draw_text(&format!("HP:{}", z.hp as i32), zx - 24.0, zy + (dest_h/2.0) + 8.0, 22.0, WHITE);
                    } else if let Some(tex) = zombie_tex {
                        let _w = tex.width();
                        let _h = tex.height();
                        let dest_w = 80.0;
                        let dest_h = 80.0;
                        draw_texture_ex(tex, zx - dest_w/2.0, zy - dest_h/2.0, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(dest_w, dest_h)), source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None });
                        draw_text(&format!("HP:{}", z.hp as i32), zx - 24.0, zy + (dest_h/2.0) + 8.0, 22.0, WHITE);
                    } else {
                        draw_rectangle(zx - 32.0, zy - 32.0, 64.0, 64.0, PURPLE);
                        draw_text(&format!("HP:{}", z.hp as i32), zx - 24.0, zy + 44.0, 22.0, WHITE);
                    }
                } else {
                    if let Some(tex) = zombie_tex {
                        let _w = tex.width();
                        let _h = tex.height();
                        let dest_w = 48.0;
                        let dest_h = 48.0;
                        draw_texture_ex(tex, zx - dest_w/2.0, zy - dest_h/2.0, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(dest_w, dest_h)), source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None });
                        draw_text(&format!("HP:{}", z.hp as i32), zx - 18.0, zy + (dest_h/2.0) + 6.0, 18.0, WHITE);
                    } else {
                        draw_rectangle(zx - 20.0, zy - 20.0, 40.0, 40.0, DARKGREEN);
                        draw_text(&format!("HP:{}", z.hp as i32), zx - 18.0, zy + 36.0, 18.0, WHITE);
                    }
                }
                
            }
            // decay per-zombie shake
            for z in zombies.iter_mut() { if z.shake > 0.0 { z.shake -= dt; } }

            // Input handling: reload, shoot, quit (pause handled earlier)
            if is_key_pressed(KeyCode::Q) { return }
            // shooting: fire bullet towards mouse on left click
            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                let px = player_x;
                let py = player_y;
                let mut dx = mx - px;
                let mut dy = my - py;
                let len = (dx*dx + dy*dy).sqrt().max(1.0);
                dx /= len; dy /= len;
                let speed = 600.0;
                // primary bullet
                bullets.push(Bullet { x: px, y: py, vx: dx*speed, vy: dy*speed });
                // shotgun extra bullets if buffed
                if shotgun_extra > 0 {
                    let spread = 0.18;
                    for i in 1..=shotgun_extra {
                        let angle = (i as f32) * spread;
                        let ca = angle.cos(); let sa = angle.sin();
                        bullets.push(Bullet { x: px, y: py, vx: (dx*ca - dy*sa)*speed, vy: (dx*sa + dy*ca)*speed });
                        bullets.push(Bullet { x: px, y: py, vx: (dx*ca + dy*sa)*speed, vy: (-dx*sa + dy*ca)*speed });
                    }
                }
                // muzzle flash for shot
                muzzle_flash_time = 0.12;
                last_shot_dx = dx;
                last_shot_dy = dy;
                // play generated shot sound
                play_sound(shot_sound, PlaySoundParams { looped: false, volume: 0.6 });
            }

            // auto-shoot handling
            if auto_shoot {
                auto_shot_timer -= dt;
                if auto_shot_timer <= 0.0 {
                    auto_shot_timer = auto_shot_interval;
                    // find zombie closest to red line (max y)
                    if let Some(target) = zombies.iter().max_by(|a, b| a.y.partial_cmp(&b.y).unwrap()) {
                        let tx = target.x;
                        let ty = target.y;
                        let px = player_x;
                        let py = player_y;
                        let mut dx = tx - px;
                        let mut dy = ty - py;
                        let len = (dx*dx + dy*dy).sqrt().max(1.0);
                        dx /= len; dy /= len;
                        let speed = 600.0;
                        bullets.push(Bullet { x: px, y: py, vx: dx*speed, vy: dy*speed });
                        if shotgun_extra > 0 {
                            let spread = 0.18;
                            for i in 1..=shotgun_extra {
                                let angle = (i as f32) * spread;
                                let ca = angle.cos(); let sa = angle.sin();
                                bullets.push(Bullet { x: px, y: py, vx: (dx*ca - dy*sa)*speed, vy: (dx*sa + dy*ca)*speed });
                                bullets.push(Bullet { x: px, y: py, vx: (dx*ca + dy*sa)*speed, vy: (-dx*sa + dy*ca)*speed });
                            }
                        }
                        // muzzle flash
                        muzzle_flash_time = 0.12;
                        last_shot_dx = dx;
                        last_shot_dy = dy;
                        play_sound(shot_sound, PlaySoundParams { looped: false, volume: 0.6 });
                    }
                }
            }

            // update bullets with horizontal bounce
            for b in bullets.iter_mut() {
                b.x += b.vx * dt;
                b.y += b.vy * dt;
                if b.x <= 0.0 {
                    b.x = 0.0;
                    b.vx = -b.vx;
                } else if b.x >= screen_width() {
                    b.x = screen_width();
                    b.vx = -b.vx;
                }
            }
            // only remove bullets that go too far vertically (top/bottom)
            bullets.retain(|b| b.y >= -200.0 && b.y <= screen_height() + 200.0);

            // collisions bullets vs zombies
            let mut new_zombies: Vec<Zombie> = Vec::new();
            for z in zombies.into_iter() {
                // create a mutable binding inside the loop to avoid `for mut z` warning
                let mut z = z;
                let mut hit = false;
                bullets.retain(|b| {
                    let dx = b.x - z.x;
                    let dy = b.y - z.y;
                    if dx*dx + dy*dy < 25.0*25.0 {
                        // hit
                        hit = true;
                        false // remove bullet
                    } else { true }
                });
                if hit {
                        let dmg = player_damage + damage_boost_amount;
                    z.hp -= dmg as f32;
                    floating_texts.push(FloatingText::new(z.x, z.y, format!("-{:.1}", dmg)));
                    // small shake for this zombie
                    z.shake = 0.25;
                }
                if z.hp <= 0.0 {
                    // grant experience instead of score
                    if z.elite { exp += 2; } else { exp += 1; }
                    // check level up: every 30 Exp -> +1 level
                    let new_level = (exp / 30) + 1;
                    if new_level > level {
                        let gained = new_level - level;
                        // for each level gained, increase player damage by 20%
                        for _ in 0..gained { player_damage *= 1.2; }
                        level = new_level;
                        // trigger level-up animation and floating text
                        level_up_anim = 0.8; // seconds of animation
                        floating_texts.push(FloatingText::new(player_x, player_y - 40.0, String::from("Level Up!")));
                    }
                    // 25% chance to drop a buff which is immediately applied
                    let r = rng.range(0, 99) as i32;
                    if r < 25 {
                        // choose buff type uniformly among 4
                        let which = rng.range(0, 3) as i32;
                        match which {
                            0 => {
                                // heal 12, clamp to max_health
                                player_health = (player_health + 12).min(max_health);
                                floating_texts.push(FloatingText::new(z.x, z.y, String::from("+12")));
                            }
                            1 => {
                                damage_boost_amount += 1.0;
                                damage_boost_timer = 8.0;
                            }
                            2 => {
                                global_bleed_dps = 2.0;
                                global_bleed_timer = 6.0;
                            }
                            _ => {
                                shotgun_extra += 2;
                                shotgun_timer = 6.0;
                            }
                        }
                    }
                } else {
                    new_zombies.push(z);
                }
            }
            zombies = new_zombies;

            // zombies reaching red line
            let mut still_zombies: Vec<Zombie> = Vec::new();
            for z in zombies.into_iter() {
                let mut z = z;
                let bottom = z.y + 20.0; // zombie half-height = 20
                if bottom >= red_line_y {
                    player_health -= 1;
                    floating_texts.push(FloatingText::new(z.x, red_line_y, String::from("-1")));
                } else { still_zombies.push(z) }
            }
            zombies = still_zombies;

            if player_health <= 0 { break }

            // update buff timers
            if damage_boost_timer > 0.0 { damage_boost_timer -= dt; if damage_boost_timer <= 0.0 { damage_boost_amount = 0.0; } }
            if global_bleed_timer > 0.0 { global_bleed_timer -= dt; if global_bleed_timer <= 0.0 { global_bleed_dps = 0.0; } }
            if shotgun_timer > 0.0 { shotgun_timer -= dt; if shotgun_timer <= 0.0 { shotgun_extra = 0; } }

            // difficulty timer: every 30s increase zombies' hp by 20% (round to integer)
            difficulty_timer += dt;
            if difficulty_timer >= 30.0 {
                difficulty_timer -= 30.0;
                difficulty_multiplier *= 1.2;
                // increment round and adjust elite chance (+2% per round), cap 100%
                round += 1;
                elite_chance = (elite_chance + 2).min(100);
                // increase all zombies' HP by 10% (round up)
                for z in zombies.iter_mut() {
                    z.hp = (z.hp * 1.10).ceil();
                }
            }

            // Zombie auto attack (only if a zombie is close to the red line)
            let close_threshold = 60.0;
            let close_zombies: Vec<usize> = zombies.iter().enumerate()
                .filter(|(_, z)| z.y + 20.0 >= red_line_y - close_threshold)
                .map(|(i, _)| i).collect();
            if !close_zombies.is_empty() {
                if rand::gen_range(0.0f32, 1.0f32) < 0.01 {
                    // rng.range is inclusive on both ends; guard against out-of-bounds when len == 1
                    let which = if close_zombies.len() == 1 {
                        0usize
                    } else {
                        rng.range(0, (close_zombies.len() as i32) - 1) as usize
                    };
                    let idx = close_zombies[which];
                    let dmg = rng.range(1, 2);
                    player_health -= dmg;
                    // use chosen zombie position for effect
                    let zx = zombies[idx].x;
                    let zy = zombies[idx].y;
                    floating_texts.push(FloatingText::new(zx - 20.0, zy - 10.0, format!("-{}", dmg)));
                }
            }

            // draw bullets with small tail
            for b in bullets.iter() {
                draw_circle(b.x, b.y, 5.0, YELLOW);
                draw_circle(b.x - b.vx * 0.02, b.y - b.vy * 0.02, 3.0, ORANGE);
            }

            // pickups are applied immediately on kill (no dropped items)

            // update floating texts
            for ft in floating_texts.iter_mut() {
                ft.update(get_frame_time());
                draw_text(&ft.text, ft.x, ft.y, ft.size, RED);
            }
            floating_texts.retain(|f| f.life > 0.0);

            next_frame().await;
        }

        // Game over screen with Try Again
        loop {
            clear_background(BLACK);
            draw_text(&format!("Game Over! Final Exp: {}", exp), screen_width()/2.0 - 220.0, screen_height()/2.0 - 40.0, 40.0, WHITE);
            // try again button
            let bw = 220.0;
            let bh = 60.0;
            let bx = screen_width()/2.0 - bw/2.0;
            let by = screen_height()/2.0 + 20.0;
            draw_rectangle(bx, by, bw, bh, DARKGRAY);
            draw_text("Try Again (T)", bx + 40.0, by + 40.0, 30.0, WHITE);
            draw_text("Press Q to quit", screen_width()/2.0 - 100.0, by + bh + 40.0, 22.0, GRAY);

            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                if mx >= bx && mx <= bx + bw && my >= by && my <= by + bh {
                    break; // restart game loop
                }
            }
            if is_key_pressed(KeyCode::T) { break }
            if is_key_pressed(KeyCode::Q) { return }
            next_frame().await
        }
    }
}
