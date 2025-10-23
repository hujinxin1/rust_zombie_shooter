use macroquad::prelude::*;
use macroquad::audio::{Sound, load_sound, load_sound_from_bytes, play_sound, PlaySoundParams};
use std::fs;
use std::path::Path;
use base64::{Engine as _, engine::general_purpose};

const BULLET_WAV_B64: &str = "UklGRh4EAABXQVZFZm10IBAAAAABAAEAgD4AAAB9AAACABAAZGF0YfoDAAD///////3///v//Pz///7+/fr8/f79//78+/v+AP/9/Pz+//3++/4A/fz7/v/8/P39AAD+/Pz9AP78/f4A//38/gD+/f3+/wD8/P7///v9AP39/f7/AP37/gD9/f3+/wD9+/4A/f39/v8A/fv+AP79/f7/APz8/v/+/f3+/wD8/P7//v39/v8A/fv+//79/f7+APz8/v/+/f39/gD8/P3//v39/f4A/Pz9//79/f3+APz8/f8A/Pz+/wD7/P3/APz8/f8A/Pz9/wD8/P3/APz8/f8A+/z9/wD8/P3/APz8/f8A/Pz9/wD8+/3/APv8/f8A/Pv9/wD7/P3+APz7/f4A+/z9/gD7/P3+APv8/f4A+/z9/gD7/P39APv8/f0A+/z9/QD7/P39APv7/f0A+/v9/QD7+/39APv7/f0A+/v9/QD7+/39APv7/f0A+/v9/QD6+/39APr7/fwA+vv9/AD6+/38APr7/fwA+vv9/AD6+/38APr7/fwA+vr9/AD6+v38APr6/fwA+fr9/AD5+v38APn6/fsA+fr9+wD5+v37APn6/fsA+fn9+wD5+f37APn5/fsA+fn9+wD5+f36APn5/foA+Pn9+gD4+f36APj5/foA+Pn9+gD4+P36APj4/foA+Pj9+gD49/36APf3/fkA9/f9+QD39/35APf3/fkA9/f9+QD39/35APb3/fkA9vf9+QD29/34APb3/fgA9vb9+AD29v34APb2/fgA9fb9+AD19v34APX2/fgA9fb99wD19v33APX1/fcA9PX99wD09f33APT1/fcA9PX99wD09f33APT1/fcA9PT99wD09P32APT0/fYA8/T99gDz9P32APP0/fYA8/P99gDz8/32APPz/fYA8/P99QDz8/31APLz/fUA8vP99QDy8v31APLy/fUA8vL99QDy8v31APLy/fUA8vL99ADx8v30APHy/fQA8fL99ADx8f30APHx/fQA8fH99ADx8f30APHx/fQA8PH98wDw8f3zAPDx/fMA8PH98wDw8P3zAPDw/fMA8PD98wDw8P3zAPDw/fIA7/D98gDv8P3yAO/w/fIA7/D98gDv7/3yAO/v/fIA7+/98gDv7/3yAO/v/fEA7+/98QDu7/3xAO7u/fEA7u798QDu7v3xAO7u/fEA7u798QDu7v3xAO7u/fEA7u794ADt7v3gAO3u/fAA7e794ADt7f3wAO3t/fAA7e398ADt7f3wAO3t/fAA7e398ADt7f3vAOzt/e8A7O397wDs7f3vAOzt/e8A7O397wDs7f3vAOzt/e8A7Oz97wDs7P3uAOzs/e4A7Oz97gDr7P3uAOvs/e4A6+z97gDr7P3uAOvr/e4A6+v97QDr6/3tAOvr/e0A6+v97QDr6/3tAOvr/e0=";

const BOOM_WAV_B64: &str = "UklGRhwFAABXQVZFZm10IBAAAAABAAEAgD4AAAB9AAACABAAZGF0YfgEAAD//wAA//8AAP//AAD+/wAA/v8AAP7/AAD9/wAA/f8AAP3/AAD8/wAA+/8AAPv/AAD6/wAA+v8AAP7/AAD9/wAA/f8AAP3/AAD8/wAA/P8AAPv/AAD7/wAA+v8AAPr/AAD5/wAA+f8AAPj/AAD4/wAA9/8AAPf/AAD2/wAA9v8AAPX/AAD1/wAA9P8AAPT/AADz/wAA8/8AAPP/AADy/wAA8v8AAPH/AADx/wAA8P8AAPD/AADv/wAA7/8AAO//AADu/wAA7v8AAO3/AADt/wAA7P8AAOz/AADr/wAA6/8AAOr/AADq/wAA6f8AAOn/AADo/wAA6P8AAOf/AADn/wAA5v8AAOb/AADl/wAA5f8AAOT/AADk/wAA4/8AAOP/AADi/wAA4v8AAOH/AADh/wAA4P8AAOD/AADf/wAA3/8AAN7/AADe/wAA3f8AAN3/AADc/wAA3P8AANv/AADb/wAA2v8AANr/AADZ/wAA2f8AANj/AADY/wAA1/8AANf/AADW/wAA1v8AANb/AADV/wAA1f8AANT/AADU/wAA0/8AANP/AADS/wAA0v8AANH/AADQ/wAA0P8AANH/AADQ/wAA0P8AAM//AADP/wAAzv8AAM7/AADM/wAAzP8AAM3/AADO/wAAzf8AAM3/AADM/wAAy/8AAMv/AADL/wAAyv8AAMr/AADK/wAAyf8AAMn/AADu/wAA7P8AAO3/AADt/wAA6/8AAOv/AADq/wAA6v8AAOr/AADp/wAA6P8AAOj/AADn/wAA5/8AAOb/AADm/wAA5f8AAOX/AADk/wAA5P8AAOP/AADj/wAA4v8AAOL/AADh/wAA4f8AAOD/AADg/wAA3/8AAN//AADe/wAA3v8AAN3/AADd/wAA3P8AANz/AADb/wAA2/8AANr/AADY/wAA2P8AANn/AADZ/wAA2P8AANj/AADX/wAA1/8AANb/AADW/wAA1f8AANX/AADU/wAA1P8AANP/AADT/wAA0v8AANL/AADR/wAA0f8AAND/AADQ/wAAz/8AAM//AADO/wAAzv8AAM3/AADN/wAAzP8AAMz/AADL/wAAy/8AAMr/AADK/wAAyf8AAMn/AADI/wAAyP8AAMf/AADH/wAAxv8AAMb/AADF/wAAxf8AAMT/AADE/wAAxP8AAM3/AADM/wAAy/8AAMr/AADK/wAAyf8AAMn/AADI/wAAyP8AAMf/AADH/wAAxv8AAMb/AADF/wAAxf8AAMT/AADE/wAAxP8AAM3/AADM/wAAy/8AAMr/AADK/wAAyf8AAMn/AADI/wAAyP8AAMf/AADH/wAAxv8AAMb/AADF/wAAxf8AAMT/AADE/wAAxP8AAM3/AADM/wAAy/8AAMr/AADK/wAAyf8AAMn/AADI/wAAyP8AAMf/AADH/wAAxv8AAMb/AADF/wAAxf8AAMT/AADE/wAAxP8AAND/AADP/wAAzv8AANH/AADQ/wAAz/8AAND/AADP/wAAzv8AAM//AADO/wAAzf8AAM7/AADN/wAAzP8AAMz/AADL/wAAy/8AAMr/AADK/wAAyf8AAMn/AADI/wAAyP8AAMf/AADH/wAAxv8AAMb/AADF/wAAxf8AAMT/AADE/wAAxP8=";

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

struct Explosion {
    x: f32,
    y: f32,
    life: f32,
    size: f32,
    radius: f32,
    max_radius: f32,
    sound_played: bool,
}
impl Explosion {
    fn new(x: f32, y: f32) -> Self { Self::big(x,y) }
    fn small(x: f32, y: f32) -> Self { Self { x, y, life: 0.45, size: 6.0, radius: 0.0, max_radius: 40.0, sound_played: false } }
    fn big(x: f32, y: f32) -> Self { Self { x, y, life: 0.9, size: 10.0, radius: 0.0, max_radius: 140.0, sound_played: false } }
}

// projectile for evolved bomb (visual travel effect)
struct Projectile {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    target_x: f32,
    target_y: f32,
    trail: Vec<Vec2>,
}

struct Zombie {
    x: f32,
    y: f32,
    hp: f32,
    speed: f32,
    shake: f32,
    elite: bool,
}

struct Boss {
    x: f32,
    y: f32,
    hp: f32,
    max_hp: f32,
    vx: f32,
    alive: bool,
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

// small explosion/boom sound generator (lower frequency, stronger noise)
async fn make_explosion_sound() -> Sound {
    let sample_rate = 44100u32;
    let dur_secs = 0.45f32;
    let n_samples = (sample_rate as f32 * dur_secs) as usize;
    let mut samples: Vec<i16> = Vec::with_capacity(n_samples);
    for i in 0..n_samples {
        let t = i as f32 / sample_rate as f32;
        // lower bass thump + noisy tail
        let env = (-4.0 * t).exp();
        let s = ( (t*220.0*2.0*std::f32::consts::PI).sin() * env * 0.9 ) as f32;
        let noise = (rand::gen_range(-0.6f32, 0.6f32) * env) as f32;
        let v = ((s + noise) * (i16::MAX as f32 * 0.4)) as i16;
        samples.push(v);
    }
    let mut wav: Vec<u8> = Vec::new();
    let datasize = (samples.len() * 2) as u32;
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36u32 + datasize).to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    let byte_rate = sample_rate * 2;
    wav.extend_from_slice(&byte_rate.to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes());
    wav.extend_from_slice(&16u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&datasize.to_le_bytes());
    for s in samples { wav.extend_from_slice(&s.to_le_bytes()); }
    load_sound_from_bytes(&wav).await.unwrap()
}

fn ensure_assets() {
    // Write sound files if they don't exist
    let p_bullet = Path::new("bullet.wav");
    let p_boom = Path::new("boom.wav");
    if !p_bullet.exists() {
        if let Ok(bytes) = general_purpose::STANDARD.decode(BULLET_WAV_B64) {
            let _ = fs::write(p_bullet, bytes);
        }
    }
    if !p_boom.exists() {
        if let Ok(bytes) = general_purpose::STANDARD.decode(BOOM_WAV_B64) {
            let _ = fs::write(p_boom, bytes);
        }
    }
    
    // small 1x1 PNG samples (base64) - will be scaled when drawn
    const PLAYER_PNG_B64: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR4nGNgYAAAAAMAASsJTYQAAAAASUVORK5CYII="; // white
    const PLAYER_PLUS_PNG_B64: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR4nGNgYAAAAAMAASsJTYQAAAAASUVORK5CYII="; // white
    const ZOMBIE_PNG_B64: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR4nGNgYAAAAAMAASsJTYQAAAAASUVORK5CYII="; // white
    const BOSS_PNG_B64: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR4nGNgYAAAAAMAASsJTYQAAAAASUVORK5CYII="; // white placeholder for boss
    let assets_dir = Path::new("assets");
    if !assets_dir.exists() {
        let _ = fs::create_dir_all(assets_dir);
    }
    let p_player = assets_dir.join("player.png");
    let p_player_plus = assets_dir.join("player+.png");
    let p_zombie = assets_dir.join("zombie.png");
    let p_boss = assets_dir.join("boss1.png");
    if !p_player.exists() {
        if let Ok(bytes) = general_purpose::STANDARD.decode(PLAYER_PNG_B64) {
            let _ = fs::write(p_player, bytes);
        }
    }
    if !p_player_plus.exists() {
        if let Ok(bytes) = general_purpose::STANDARD.decode(PLAYER_PLUS_PNG_B64) {
            let _ = fs::write(p_player_plus, bytes);
        }
    }
    if !p_zombie.exists() {
        if let Ok(bytes) = general_purpose::STANDARD.decode(ZOMBIE_PNG_B64) {
            let _ = fs::write(p_zombie, bytes);
        }
    }
    if !p_boss.exists() {
        if let Ok(bytes) = general_purpose::STANDARD.decode(BOSS_PNG_B64) {
            let _ = fs::write(p_boss, bytes);
        }
    }

    // if files exist but are invalid, remove them so loader won't attempt to parse corrupted wavs
    if p_bullet.exists() && !wav_is_valid(p_bullet) {
        let _ = fs::remove_file(p_bullet);
    }
    if p_boom.exists() && !wav_is_valid(p_boom) {
        let _ = fs::remove_file(p_boom);
    }
}

// validate wav file quickly (module-level so other code can call it)
fn wav_is_valid(p: &Path) -> bool {
    use std::io::Read;
    if !p.exists() { return false }
    let mut f = match fs::File::open(p) { Ok(f) => f, Err(_) => return false };
    let mut header = [0u8; 12];
    if f.read_exact(&mut header).is_err() { return false }
    if &header[0..4] != b"RIFF" { return false }
    if &header[8..12] != b"WAVE" { return false }
    // scan subchunks until we find 'fmt ' and 'data'
    let mut found_fmt = false;
    let mut fmt_size: u32 = 0;
    let mut found_data = false;
    let mut data_len: u32 = 0;
    let mut buf = [0u8;8];
    loop {
        if f.read_exact(&mut buf).is_err() { break }
        let id = &buf[0..4];
        let size = u32::from_le_bytes([buf[4],buf[5],buf[6],buf[7]]);
        if id == b"fmt " {
            found_fmt = true;
            fmt_size = size;
            // skip fmt payload
            if f.by_ref().take(size as u64).read_to_end(&mut Vec::new()).is_err() { return false }
        } else if id == b"data" {
            found_data = true;
            data_len = size;
            break;
        } else {
            // skip unknown chunk
            if f.by_ref().take(size as u64).read_to_end(&mut Vec::new()).is_err() { return false }
        }
    }
    if !found_data { return false }
    // ensure file length is at least header + fmt chunk + data chunk
    let mut remaining = 0u64;
    if let Ok(metadata) = p.metadata() { remaining = metadata.len(); }
    // minimal required bytes: 12 (RIFF header) + (we don't know exact positions of fmt and others) + data_len <= file len
    // approximate check: file must be at least data_len + 44
    let required = 44u64 + data_len as u64;
    remaining >= required
}

// Diagnostic reader: return header(12), data_len, file_len for debugging
fn wav_diagnose(p: &Path) -> Option<([u8;12], u32, u64)> {
    use std::io::Read;
    if !p.exists() { return None }
    let mut f = fs::File::open(p).ok()?;
    let mut header = [0u8;12];
    f.read_exact(&mut header).ok()?;
    // scan chunks until data
    let mut buf = [0u8;8];
    loop {
        if f.read_exact(&mut buf).is_err() { return None }
        let id = &buf[0..4];
        let size = u32::from_le_bytes([buf[4],buf[5],buf[6],buf[7]]);
        if id == b"data" {
            let data_len = size;
            let file_len = p.metadata().ok()?.len();
            return Some((header, data_len, file_len));
        } else {
            // skip this chunk
            if f.by_ref().take(size as u64).read_to_end(&mut Vec::new()).is_err() { return None }
        }
    }
}

// print first n bytes as hex for debugging
fn wav_hexdump(p: &Path, n: usize) -> Option<String> {
    use std::io::Read;
    let mut f = fs::File::open(p).ok()?;
    let mut buf = vec![0u8; n];
    let read = f.read(&mut buf).ok()?;
    buf.truncate(read);
    let s = buf.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ");
    Some(s)
}

struct Bullet {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

const BOSS_BASE_RADIUS: f32 = 40.0;
const ZOMBIE_BASE_RADIUS: f32 = 25.0;
const BOSS_VISUAL_SCALE: f32 = 1.5;


#[macroquad::main("Zombie Shooter")]
async fn main() {
    // ensure assets exist (write sample PNGs if missing)
    ensure_assets();
    // outer loop allows restarting the game (Try Again)
    loop {
        let mut rng = Lcg::new(123456789);
        // try loading textures (assets/player.png, assets/zombie.png)
        let mut player_tex = load_texture("assets/player.png").await.ok();
        let player_plus_tex = load_texture("assets/player+.png").await.ok();
        let zombie_tex = load_texture("assets/zombie.png").await.ok();
        let elite_zombie_tex = load_texture("assets/zombie2.png").await.ok();
        let boss_tex = load_texture("assets/boss1.png").await.ok();
        let mut max_health: i32 = 100;
        let mut player_health: i32 = max_health;
        // unlimited bullets
    let mut exp: i32 = 0;
    let mut round: i32 = 1;
    let mut level: i32 = 1;
    let _zombie_health: i32 = rng.range(2, 4);
    let mut floating_texts: Vec<FloatingText> = Vec::new();
    let mut explosions: Vec<Explosion> = Vec::new();
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
    let mut evolution_anim: f32 = 0.0;
    let mut has_evolved: bool = false;
    // skill cooldown state (Burst)
    let mut skill_cooldown: f32 = 0.0;
    let skill_cooldown_duration: f32 = 15.0; // shortened to 15s

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
    // Boss state
    let mut boss: Option<Boss> = None;

    let mut boss_appear_count: i32 = 0; // counts how many times boss appeared (for scaling)
    let mut boss_fire_timer: f32 = 0.0; // track boss firing every 3s when alive
    let boss_fire_interval: f32 = 3.0;
    // evolved auto-fire (every 5s) targeting enemy nearest to red line
    let mut evolved_fire_timer: f32 = 0.0;
    let evolved_fire_interval: f32 = 5.0;
    // red_line_y, player_x, player_y will be computed each frame to adapt to screen and player size

    // prepare sounds (try to load external assets first, fallback to generated sounds)
    // only attempt to load external file if it's a WAV or OGG to avoid unsupported-format panics
    fn is_supported_sound_file(path: &str) -> bool {
        let p = Path::new(path);
        if !p.exists() { return false }
        if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
            let e = ext.to_lowercase();
            return e == "wav" || e == "ogg";
        }
        false
    }
    // Try loading valid WAV files from working directory first, then assets/, otherwise fallback to generated sounds
    async fn try_load_valid_wav(path: &str) -> Option<Sound> {
        let p = Path::new(path);
        if p.exists() {
            // quick local validation before handing to load_sound
            if wav_is_valid(p) {
                match load_sound(path).await {
                    Ok(s) => return Some(s),
                    Err(e) => {
                        println!("load_sound failed for {}: {:?}", path, e);
                        if let Some((hdr, data_len, file_len)) = wav_diagnose(p) {
                            println!("DIAG {} header={:?} data_len={} file_len={}", path, &hdr, data_len, file_len);
                        } else {
                            println!("DIAG {}: failed to diagnose file", path);
                        }
                    }
                }
            } else {
                println!("Validation failed for {}", path);
                if let Some((hdr, data_len, file_len)) = wav_diagnose(p) {
                    println!("DIAG {} header={:?} data_len={} file_len={}", path, &hdr, data_len, file_len);
                } else {
                    println!("DIAG {}: failed to diagnose file", path);
                    if let Some(hex) = wav_hexdump(p, 128) {
                        println!("HEXDUMP {}: {}", path, hex);
                    }
                }
            }
        }
        None
    }

    // check working dir
    let shot_sound = if let Some(s) = try_load_valid_wav("bullet.wav").await { s }
    else if let Some(s) = try_load_valid_wav("assets/bullet.wav").await { s }
    else { make_shot_sound().await };

    let explosion_sound = if let Some(s) = try_load_valid_wav("boom.wav").await { s }
    else if let Some(s) = try_load_valid_wav("assets/boom.wav").await { s }
    else { make_explosion_sound().await };
    // projectiles (visual) for evolved bombs
    let mut projectiles: Vec<Projectile> = Vec::new();

        loop {
            clear_background(LIGHTGRAY);
            let dt = get_frame_time();

            // compute player draw size and positions adaptive to screen size (used by HUD and other logic)
            let mut player_dest_w = 64.0;
            let mut player_dest_h = 64.0;
            // level up animation scaling
            if level_up_anim > 0.0 {
                let s = 1.0 + (level_up_anim * 0.5);
                player_dest_w *= s;
                player_dest_h *= s;
            }
            // larger when evolved (2.5x)
            if has_evolved {
                player_dest_w *= 2.5;
                player_dest_h *= 2.5;
            }
            let red_line_y = screen_height() * 0.75;
            let player_x = screen_width() / 2.0;
            let player_y = red_line_y + (player_dest_h * 0.5) + 16.0;

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
            // skill button (right-middle) - circular
            let skill_r = 40.0; // radius
            let skill_x = screen_width() - skill_r - 12.0; // circle center x (near right edge)
            let skill_y = screen_height() / 2.0; // circle center y
            // update skill cooldown using main loop dt
            if skill_cooldown > 0.0 {
                skill_cooldown -= dt;
                if skill_cooldown < 0.0 { skill_cooldown = 0.0; }
            }
            // Draw circular skill button and circular cooldown progress
            // base circle background (always drawn in original color)
            let base_color = if skill_cooldown > 0.0 { DARKGRAY } else { LIGHTGRAY };
            draw_circle(skill_x, skill_y, skill_r, base_color);
            // draw label centered (on top)
            let text = "Burst";
            let m = measure_text(text, None, 22, 1.0);
            // draw overlay for remaining cooldown as a semi-transparent black sector (so uncovered part stays original color)
            if skill_cooldown > 0.0 {
                let frac = skill_cooldown / skill_cooldown_duration; // 1.0 -> full, 0.0 -> ready
                let segments = 48; // smoothness
                let two_pi = std::f32::consts::PI * 2.0;
                // we want to cover the portion from angle = 0..(two_pi * frac)
                let cover_angle = two_pi * frac;
                let step = cover_angle / (segments as f32);
                let mut last_a = 0.0f32;
                for i in 1..=segments {
                    let a = step * (i as f32);
                    let x1 = skill_x + skill_r * last_a.cos();
                    let y1 = skill_y + skill_r * last_a.sin();
                    let x2 = skill_x + skill_r * a.cos();
                    let y2 = skill_y + skill_r * a.sin();
                    draw_triangle(
                        Vec2::new(skill_x, skill_y),
                        Vec2::new(x1, y1),
                        Vec2::new(x2, y2),
                        Color::new(0.0, 0.0, 0.0, 0.45),
                    );
                    last_a = a;
                }
                // draw outline ring
                draw_circle_lines(skill_x, skill_y, skill_r + 1.0, 2.0, Color::new(0.6, 0.6, 0.6, 1.0));
            } else {
                // ready outline (green)
                draw_circle_lines(skill_x, skill_y, skill_r + 1.0, 2.0, Color::new(0.2, 0.8, 0.2, 1.0));
            }
            // draw label last so it remains readable above overlays
            draw_text(text, skill_x - m.width / 2.0, skill_y + m.height / 2.0, 22.0, WHITE);
            // show evolved AOE cooldown near skill button
            if has_evolved {
                let cd_remain = (evolved_fire_interval - evolved_fire_timer).max(0.0);
                // small circle below the skill button
                let e_x = skill_x;
                let e_y = skill_y + skill_r + 28.0;
                draw_circle(e_x, e_y, 18.0, DARKGRAY);
                // draw cooldown arc
                if cd_remain > 0.01 {
                    let frac = cd_remain / evolved_fire_interval;
                    let segments = 36;
                    let two_pi = std::f32::consts::PI * 2.0;
                    let cover_angle = two_pi * frac;
                    let step = cover_angle / (segments as f32);
                    let mut last_a = 0.0f32;
                    for i in 1..=segments {
                        let a = step * (i as f32);
                        let x1 = e_x + 18.0 * last_a.cos();
                        let y1 = e_y + 18.0 * last_a.sin();
                        let x2 = e_x + 18.0 * a.cos();
                        let y2 = e_y + 18.0 * a.sin();
                        draw_triangle(Vec2::new(e_x, e_y), Vec2::new(x1, y1), Vec2::new(x2, y2), Color::new(0.0, 0.0, 0.0, 0.5));
                        last_a = a;
                    }
                }
                draw_text(&format!("{:.0}s", cd_remain.ceil()), e_x - 12.0, e_y + 6.0, 18.0, WHITE);
            }
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
                // skill button click: Burst (circle hit test)
                let dx = mx - skill_x;
                let dy = my - skill_y;
                if dx*dx + dy*dy <= skill_r*skill_r {
                    if skill_cooldown <= 0.0 {
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
                                // spawn small explosion on death
                                explosions.push(Explosion::small(z.x, z.y));
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
                        // set cooldown after using skill
                        skill_cooldown = skill_cooldown_duration;
                    }
                }
            }

            // draw red line and player
            draw_line(0.0, red_line_y, screen_width(), red_line_y, 4.0, RED);
            if let Some(tex) = player_tex {
                let _w = tex.width();
                let _h = tex.height();
                draw_texture_ex(tex, player_x - player_dest_w/2.0, player_y - player_dest_h/2.0, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(player_dest_w, player_dest_h)), source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None });
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
            
            // Evolution animation
            if level >= 5 && !has_evolved {
                evolution_anim = 1.0;
                has_evolved = true;
                // Switch texture
                if let Some(plus_tex) = player_plus_tex {
                    player_tex = Some(plus_tex);
                }
                // Apply evolution buffs
                player_damage += 10.0;
                let old_max_health = max_health;
                max_health += 50;
                player_health += max_health - old_max_health;
                floating_texts.push(FloatingText::new(player_x, player_y - 60.0, String::from("Evolved!")));
            }
            
            // Evolution animation effect
            if evolution_anim > 0.0 {
                let s = 1.0 + (evolution_anim * 1.0); // double size at peak
                let alpha = evolution_anim;
                draw_circle(player_x, player_y, 48.0 * s, Color::new(1.0, 1.0, 0.0, alpha));
                evolution_anim -= dt;
            }

            // evolved auto-fire: when evolved, tick timer and fire every evolved_fire_interval (spawn projectile)
            if has_evolved {
                evolved_fire_timer += dt;
                if evolved_fire_timer >= evolved_fire_interval {
                    evolved_fire_timer = 0.0;
                    // find zombie whose vertical distance to red_line_y is minimal
                    if !zombies.is_empty() {
                        let mut best_idx: Option<usize> = None;
                        let mut best_dist = std::f32::INFINITY;
                        for (i, z) in zombies.iter().enumerate() {
                            let d = (z.y - red_line_y).abs();
                            if d < best_dist {
                                best_dist = d;
                                best_idx = Some(i);
                            }
                        }
                        if let Some(idx) = best_idx {
                            let target_x = zombies[idx].x;
                            let target_y = zombies[idx].y;
                            // spawn a projectile from player towards target
                            let px = player_x;
                            let py = player_y - player_dest_h*0.5;
                            let dir_x = target_x - px;
                            let dir_y = target_y - py;
                            let len = (dir_x*dir_x + dir_y*dir_y).sqrt().max(0.001);
                            let speed = 480.0;
                            // travel time estimate
                            let t = len / speed;
                            let g = 800.0; // gravity (pixels/s^2)
                            // compute initial velocities to reach target with gravity
                            let vx = dir_x / t;
                            let vy = (dir_y - 0.5 * g * t * t) / t;
                            projectiles.push(Projectile { x: px, y: py, vx, vy, life: t + 0.6, target_x, target_y, trail: Vec::new() });
                            // muzzle flash and firing sound
                            muzzle_flash_time = 0.12;
                            play_sound(shot_sound, PlaySoundParams { looped: false, volume: 0.8 });
                            // play firing sound (more realistic)
                            play_sound(shot_sound, PlaySoundParams { looped: false, volume: 0.8 });
                        }
                    }
                }
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
                // Calculate health with 50% increase per round
                let round_multiplier = (1.5f32).powi(round - 1);
                let mut hp = (base_zhp * difficulty_multiplier * round_multiplier).round();
                if is_elite { hp = (hp * 2.0).round(); }
                zombies.push(Zombie { x: zx, y: -40.0, hp: hp as f32, speed: zspeed, shake: 0.0, elite: is_elite });
            }

            // Boss spawn on round 5
            if round == 5 && boss.is_none() {
                // spawn boss in the middle top
                boss_appear_count += 1;
                let mut hp = 300.0 * (2.0f32).powi(boss_appear_count - 1);
                boss = Some(Boss { x: screen_width()/2.0, y: 40.0, hp: hp, max_hp: hp, vx: 120.0, alive: true });
                // start firing timer
                boss_fire_timer = 0.0;
                floating_texts.push(FloatingText::new(screen_width()/2.0, 20.0, String::from("BOSS APPEARED!")));
            }

            // update boss behavior if present
            if let Some(ref mut b) = boss {
                if b.alive {
                    // keep boss at top area y fixed (don't move down)
                    if b.y < 40.0 { b.y = 40.0; }
                    // horizontal movement
                    b.x += b.vx * dt;
                    // bounce on screen edges
                    if b.x <= 60.0 { b.x = 60.0; b.vx = b.vx.abs(); }
                    if b.x >= screen_width() - 60.0 { b.x = screen_width() - 60.0; b.vx = -b.vx.abs(); }
                    // boss firing
                    boss_fire_timer += dt;
                    if boss_fire_timer >= boss_fire_interval {
                        boss_fire_timer -= boss_fire_interval;
                        // compute scaled damage based on appear count
                        let dmg = 5.0 * (2.0f32).powi(boss_appear_count - 1);
                        // instant laser visual
                        explosions.push(Explosion::new(b.x, b.y + 20.0));
                        // instantly apply damage to player
                        player_health -= dmg as i32;
                        floating_texts.push(FloatingText::new(player_x, player_y - 20.0, format!("-{}", dmg as i32)));
                    }
                }
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

            // draw boss if present
            if let Some(ref b) = boss {
                if b.alive {
                    if let Some(tex) = boss_tex {
                        let dest_w = 140.0 * BOSS_VISUAL_SCALE; // increased size
                        let dest_h = 80.0 * BOSS_VISUAL_SCALE;
                        draw_texture_ex(tex, b.x - dest_w/2.0, b.y - dest_h/2.0, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(dest_w, dest_h)), source: None, rotation: 0.0, flip_x: false, flip_y: false, pivot: None });
                        draw_text(&format!("Boss HP:{}", b.hp as i32), b.x - 60.0, b.y + dest_h/2.0 + 12.0, 20.0, RED);
                    } else {
                        draw_rectangle(b.x - 70.0 * BOSS_VISUAL_SCALE, b.y - 30.0 * BOSS_VISUAL_SCALE, 140.0 * BOSS_VISUAL_SCALE, 60.0 * BOSS_VISUAL_SCALE, MAROON);
                        draw_text(&format!("Boss HP:{}", b.hp as i32), b.x - 60.0, b.y + 32.0, 20.0, RED);
                    }
                }
            }

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

            // bullets vs boss
            if let Some(ref mut b) = boss {
                if b.alive {
                    // iterate bullets and check collision with boss (use radius 40)
                    let mut remaining_bullets: Vec<Bullet> = Vec::new();
                    for bl in bullets.drain(..) {
                        let dx = bl.x - b.x;
                        let dy = bl.y - b.y;
                        let boss_hit_radius = BOSS_BASE_RADIUS * BOSS_VISUAL_SCALE;
                        if dx*dx + dy*dy < boss_hit_radius*boss_hit_radius {
                            // hit boss
                            let dmg = (player_damage + damage_boost_amount) as f32;
                            b.hp -= dmg;
                            floating_texts.push(FloatingText::new(b.x, b.y, format!("-{:.1}", dmg)));
                            explosions.push(Explosion::new(b.x, b.y));
                            // don't push bullet back (it is consumed)
                        } else {
                            remaining_bullets.push(bl);
                        }
                    }
                    bullets = remaining_bullets;
                        // check boss death
                    if b.hp <= 0.0 {
                        b.alive = false;
                        explosions.push(Explosion::new(b.x, b.y));
                        // grant 100 exp and permanent buffs
                        exp += 100;
                        // Permanent buffs: damage +10, health +50
                        player_damage += 10.0;
                        let old_max_health = max_health;
                        max_health += 50;
                        player_health += max_health - old_max_health;
                        floating_texts.push(FloatingText::new(b.x, b.y, String::from("Boss Down!")));
                        floating_texts.push(FloatingText::new(b.x, b.y + 30.0, String::from("+100 Exp")));
                        floating_texts.push(FloatingText::new(player_x, player_y - 40.0, String::from("Permanent Buff!")));
                        round += 1;
                        boss = None;
                    }
                }
            }
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
                    // spawn small explosion on death
                    explosions.push(Explosion::small(z.x, z.y));
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

            // update projectiles (visual travel) for evolved bombs
            let g = 800.0;
            for p in projectiles.iter_mut() {
                p.life -= dt;
                // gravity
                p.vy += g * dt;
                p.x += p.vx * dt;
                p.y += p.vy * dt;
                // trail
                p.trail.push(Vec2::new(p.x, p.y));
                if p.trail.len() > 12 { p.trail.remove(0); }
                // draw trail with fading
                let mut alpha = 0.2;
                let step = 0.8 / (p.trail.len() as f32 + 0.01);
                for pos in p.trail.iter() {
                    draw_circle(pos.x, pos.y, 4.0, Color::new(1.0, 0.6, 0.15, alpha));
                    alpha += step;
                }
                // draw projectile head
                draw_circle(p.x, p.y, 6.0, Color::new(0.95, 0.75, 0.25, 1.0));
            }
            // check projectiles arrival
            let mut remaining_projectiles: Vec<Projectile> = Vec::new();
            for p in projectiles.into_iter() {
                let dx = p.x - p.target_x;
                let dy = p.y - p.target_y;
                if (dx*dx + dy*dy).sqrt() <= 18.0 || p.life <= 0.0 {
                    // explode here: create big explosion, apply AOE damage and play sound
                    explosions.push(Explosion::new(p.target_x, p.target_y));
                    play_sound(explosion_sound, PlaySoundParams { looped: false, volume: 0.9 });
                    // apply damage similar to previous: aoe radius and damage
                    let aoe_radius = 120.0;
                    let aoe_damage = 20.0 + (level as f32 * 2.0);
                    let mut new_zombies: Vec<Zombie> = Vec::new();
                    for mut z in zombies.into_iter() {
                        let dx2 = z.x - p.target_x;
                        let dy2 = z.y - p.target_y;
                        let dist = (dx2*dx2 + dy2*dy2).sqrt();
                        if dist <= aoe_radius {
                            z.hp -= aoe_damage;
                            floating_texts.push(FloatingText::new(z.x, z.y - 10.0, format!("-{:.1}", aoe_damage)));
                            if z.hp <= 0.0 {
                                explosions.push(Explosion::small(z.x, z.y));
                                if z.elite { exp += 2; } else { exp += 1; }
                                // 25% chance buff drop on AOE kill
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
                                let new_level = (exp / 30) + 1;
                                if new_level > level {
                                    let gained = new_level - level;
                                    for _ in 0..gained { player_damage *= 1.2; }
                                    level = new_level;
                                    level_up_anim = 0.8;
                                    floating_texts.push(FloatingText::new(player_x, player_y - 40.0, String::from("Level Up!")));
                                }
                                continue;
                            }
                        }
                        new_zombies.push(z);
                    }
                    zombies = new_zombies;
                } else {
                    remaining_projectiles.push(p);
                }
            }
            projectiles = remaining_projectiles;

            // update & draw explosions (animated radius/size)
            for ex in explosions.iter_mut() {
                ex.life -= dt;
                // animate radius expanding
                ex.radius = ex.radius + (ex.max_radius * dt / 0.25);
                if ex.radius > ex.max_radius { ex.radius = ex.max_radius; }
                ex.size += 120.0 * dt;
                let alpha = (ex.life / 0.9).max(0.0);
                // outer shock
                draw_circle_lines(ex.x, ex.y, ex.radius, 6.0 * alpha, Color::new(1.0, 0.6, 0.1, 0.9 * alpha));
                // core
                draw_circle(ex.x, ex.y, ex.size * 0.6, Color::new(1.0, 0.7, 0.0, alpha));
                draw_circle(ex.x, ex.y, ex.size, Color::new(1.0, 0.45, 0.0, alpha * 0.75));
            }
            explosions.retain(|e| e.life > 0.0);

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
