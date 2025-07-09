use gameboy_dmg_emulator::{GameBoy, EmulatorError};

#[test]
fn test_gameboy_creation() {
    let gameboy = GameBoy::new();
    assert_eq!(gameboy.get_frame_buffer().len(), 0);
    assert_eq!(gameboy.get_audio_samples().len(), 0);
}

#[test]
fn test_rom_loading() {
    let mut gameboy = GameBoy::new();
    
    // Test with empty ROM (should fail)
    let empty_rom = vec![];
    assert!(gameboy.load_rom(&empty_rom).is_err());
    
    // Test with minimal ROM
    let minimal_rom = vec![0; 0x8000];
    assert!(gameboy.load_rom(&minimal_rom).is_ok());
}

#[test]
fn test_emulator_step() {
    let mut gameboy = GameBoy::new();
    
    // Should not panic
    gameboy.step();
    gameboy.step();
    gameboy.step();
}