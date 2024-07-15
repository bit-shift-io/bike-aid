#![allow(dead_code)]
use crate::note::*;

// songs: https://github.com/robsoncouto/arduino-songs/tree/master

pub const ASABRANCA_TEMPO: i32 = 120;
pub const ASABRANCA: [isize; 184] = [
    // Asa branca - Luiz Gonzaga
    // Score available at https://musescore.com/user/190926/scores/181370
    NOTE_G4,8, NOTE_A4,8, NOTE_B4,4, NOTE_D5,4, NOTE_D5,4, NOTE_B4,4, 
    NOTE_C5,4, NOTE_C5,2, NOTE_G4,8, NOTE_A4,8,
    NOTE_B4,4, NOTE_D5,4, NOTE_D5,4, NOTE_C5,4,
  
    NOTE_B4,2, REST,8, NOTE_G4,8, NOTE_G4,8, NOTE_A4,8,
    NOTE_B4,4, NOTE_D5,4, REST,8, NOTE_D5,8, NOTE_C5,8, NOTE_B4,8,
    NOTE_G4,4, NOTE_C5,4, REST,8, NOTE_C5,8, NOTE_B4,8, NOTE_A4,8,
  
    NOTE_A4,4, NOTE_B4,4, REST,8, NOTE_B4,8, NOTE_A4,8, NOTE_G4,8,
    NOTE_G4,2, REST,8, NOTE_G4,8, NOTE_G4,8, NOTE_A4,8,
    NOTE_B4,4, NOTE_D5,4, REST,8, NOTE_D5,8, NOTE_C5,8, NOTE_B4,8,
  
    NOTE_G4,4, NOTE_C5,4, REST,8, NOTE_C5,8, NOTE_B4,8, NOTE_A4,8,
    NOTE_A4,4, NOTE_B4,4, REST,8, NOTE_B4,8, NOTE_A4,8, NOTE_G4,8,
    NOTE_G4,4, NOTE_F5,8, NOTE_D5,8, NOTE_E5,8, NOTE_C5,8, NOTE_D5,8, NOTE_B4,8,
  
    NOTE_C5,8, NOTE_A4,8, NOTE_B4,8, NOTE_G4,8, NOTE_A4,8, NOTE_G4,8, NOTE_E4,8, NOTE_G4,8,
    NOTE_G4,4, NOTE_F5,8, NOTE_D5,8, NOTE_E5,8, NOTE_C5,8, NOTE_D5,8, NOTE_B4,8,
    NOTE_C5,8, NOTE_A4,8, NOTE_B4,8, NOTE_G4,8, NOTE_A4,8, NOTE_G4,8, NOTE_E4,8, NOTE_G4,8,
    NOTE_G4,-2, REST,4
];


pub const RYDE_OF_THE_WALKYRIES_TEMPO: i32 = 84;
pub const RYDE_OF_THE_WALKYRIES: [isize; 78] = [
    NOTE_B3, 4, NOTE_FS3, 8, NOTE_B3, 4, NOTE_D4, -1, NOTE_B3, -1, 
    NOTE_D4, 4, NOTE_B3, 8, NOTE_D4, 4, NOTE_FS4, -1, NOTE_D4, -1, 
    NOTE_FS4, 4, NOTE_D4, 8, NOTE_FS4, 4, NOTE_A4, -1, NOTE_A3, -1, 
    NOTE_D4, 4, NOTE_A3, 8, NOTE_D4, 4, NOTE_FS4, 1,

    NOTE_B3, 4, NOTE_D4, 8, NOTE_B3, 4, NOTE_D4, -1, NOTE_FS4, -1, 
    NOTE_D4, 4, NOTE_FS4, 8, NOTE_D4, 4, NOTE_FS4, -1, NOTE_A4, -1, 
    NOTE_FS4, 4, NOTE_A4, 8, NOTE_FS4, 4, NOTE_A4, -1, NOTE_CS5, -1, 
    NOTE_CS4, 4, NOTE_FS4, 8, NOTE_CS4, 4, NOTE_FS4, 1,  NOTE_A4, 1
];
/*
pub const Ryde_Of_The_Walkyries = [
    4, 8, 4, 1.34, 1.34, 
    4, 8, 4, 1.34, 1.34, 
    4, 8, 4, 1.34, 1.34, 
    4, 8, 4, 0.83, 

    4, 4, 8, 4, 1.34, 1.34,
    4, 8, 4, 1.34, 1.34,
     4, 8, 4, 1.34, 1.34, 
     4, 8, 4, 0.30
]; */

    

pub const STAR_WARS_TEMPO: i32 = 108;
pub const STAR_WARS: [isize; 176] = [
    // Dart Vader theme (Imperial March) - Star wars 
    // Score available at https://musescore.com/user/202909/scores/1141521
    // The tenor saxophone part was used
    NOTE_AS4,8, NOTE_AS4,8, NOTE_AS4,8,//1
    NOTE_F5,2, NOTE_C6,2,
    NOTE_AS5,8, NOTE_A5,8, NOTE_G5,8, NOTE_F6,2, NOTE_C6,4,  
    NOTE_AS5,8, NOTE_A5,8, NOTE_G5,8, NOTE_F6,2, NOTE_C6,4,  
    NOTE_AS5,8, NOTE_A5,8, NOTE_AS5,8, NOTE_G5,2, NOTE_C5,8, NOTE_C5,8, NOTE_C5,8,
    NOTE_F5,2, NOTE_C6,2,
    NOTE_AS5,8, NOTE_A5,8, NOTE_G5,8, NOTE_F6,2, NOTE_C6,4,  
    
    NOTE_AS5,8, NOTE_A5,8, NOTE_G5,8, NOTE_F6,2, NOTE_C6,4, //8  
    NOTE_AS5,8, NOTE_A5,8, NOTE_AS5,8, NOTE_G5,2, NOTE_C5,-8, NOTE_C5,16, 
    NOTE_D5,-4, NOTE_D5,8, NOTE_AS5,8, NOTE_A5,8, NOTE_G5,8, NOTE_F5,8,
    NOTE_F5,8, NOTE_G5,8, NOTE_A5,8, NOTE_G5,4, NOTE_D5,8, NOTE_E5,4,NOTE_C5,-8, NOTE_C5,16,
    NOTE_D5,-4, NOTE_D5,8, NOTE_AS5,8, NOTE_A5,8, NOTE_G5,8, NOTE_F5,8,
    
    NOTE_C6,-8, NOTE_G5,16, NOTE_G5,2, REST,8, NOTE_C5,8,//13
    NOTE_D5,-4, NOTE_D5,8, NOTE_AS5,8, NOTE_A5,8, NOTE_G5,8, NOTE_F5,8,
    NOTE_F5,8, NOTE_G5,8, NOTE_A5,8, NOTE_G5,4, NOTE_D5,8, NOTE_E5,4,NOTE_C6,-8, NOTE_C6,16,
    NOTE_F6,4, NOTE_DS6,8, NOTE_CS6,4, NOTE_C6,8, NOTE_AS5,4, NOTE_GS5,8, NOTE_G5,4, NOTE_F5,8,
    NOTE_C6,1
];


pub const PACMAN_TEMPO: i32 = 120;
pub const PACMAN: [isize; 62] = [
    // Pacman
    // Score available at https://musescore.com/user/85429/scores/107109
    NOTE_B4, 16, NOTE_B5, 16, NOTE_FS5, 16, NOTE_DS5, 16, //1
    NOTE_B5, 32, NOTE_FS5, -16, NOTE_DS5, 8, NOTE_C5, 16,
    NOTE_C6, 16, NOTE_G6, 16, NOTE_E6, 16, NOTE_C6, 32, NOTE_G6, -16, NOTE_E6, 8,
  
    NOTE_B4, 16,  NOTE_B5, 16,  NOTE_FS5, 16,   NOTE_DS5, 16,  NOTE_B5, 32,  //2
    NOTE_FS5, -16, NOTE_DS5, 8,  NOTE_DS5, 32, NOTE_E5, 32,  NOTE_F5, 32,
    NOTE_F5, 32,  NOTE_FS5, 32,  NOTE_G5, 32,  NOTE_G5, 32, NOTE_GS5, 32,  NOTE_A5, 16, NOTE_B5, 8
];


pub const NOKIA_TEMPO: i32 = 180;
pub const NOKIA: [isize; 26] = [
    // Nokia Ringtone 
    // Score available at https://musescore.com/user/29944637/scores/5266155
    NOTE_E5, 8, NOTE_D5, 8, NOTE_FS4, 4, NOTE_GS4, 4, 
    NOTE_CS5, 8, NOTE_B4, 8, NOTE_D4, 4, NOTE_E4, 4, 
    NOTE_B4, 8, NOTE_A4, 8, NOTE_CS4, 4, NOTE_E4, 4,
    NOTE_A4, 2, 
];


pub const STAR_TREK_TEMPO: i32 = 80;
pub const STAR_TREK: [isize; 16] = [
    // Star Trek Intro
    // Score available at https://musescore.com/user/10768291/scores/4594271
    NOTE_D4, -8, NOTE_G4, 16, NOTE_C5, -4, 
    NOTE_B4, 8, NOTE_G4, -16, NOTE_E4, -16, NOTE_A4, -16,
    NOTE_D5, 2,
];


pub const MARIO_UNDERWORLD_TEMPO: i32 = 120;
pub const MARIO_UNDERWORLD: [isize; 112] = [
    // Mario underworld
    NOTE_C4, 12, NOTE_C5, 12, NOTE_A3, 12, NOTE_A4, 12 ,NOTE_AS3, 12, NOTE_AS4, 12, REST, 6, REST, 3,
    NOTE_C4, 12, NOTE_C5, 12, NOTE_A3, 12, NOTE_A4, 12, NOTE_AS3, 12, NOTE_AS4, 12, REST, 6, REST, 3,
    NOTE_F3, 12, NOTE_F4, 12, NOTE_D3, 12, NOTE_D4, 12, NOTE_DS3, 12, NOTE_DS4, 12, REST, 6, REST, 3,
    NOTE_F3, 12, NOTE_F4, 12, NOTE_D3, 12, NOTE_D4, 12 ,NOTE_DS3, 12, NOTE_DS4, 12, REST, 6, REST, 6, 
    NOTE_DS4, 18, NOTE_CS4, 18, NOTE_D4, 18,
    NOTE_CS4, 6, NOTE_DS4, 6, NOTE_DS4, 6, NOTE_GS3, 6, NOTE_G3, 6, NOTE_CS4, 6,
    NOTE_C4, 18, NOTE_FS4, 18, NOTE_F4, 18, NOTE_E3, 18, NOTE_AS4, 18, NOTE_A4, 18,
    NOTE_GS4, 10, NOTE_DS4, 10, NOTE_B3, 10, NOTE_AS3, 10, NOTE_A3, 10, NOTE_GS3, 10, REST, 3, REST, 3, REST, 3
];


pub const SUPER_MARIO_TEMPO: i32 = 200;
pub const SUPER_MARIO_BROS: [isize; 642] = [
    // Super Mario Bros theme
    // Score available at https://musescore.com/user/2123/scores/2145
    // Theme by Koji Kondo
    NOTE_E5,8, NOTE_E5,8, REST,8, NOTE_E5,8, REST,8, NOTE_C5,8, NOTE_E5,8, //1
    NOTE_G5,4, REST,4, NOTE_G4,8, REST,4, 
    NOTE_C5,-4, NOTE_G4,8, REST,4, NOTE_E4,-4, // 3
    NOTE_A4,4, NOTE_B4,4, NOTE_AS4,8, NOTE_A4,4,
    NOTE_G4,-8, NOTE_E5,-8, NOTE_G5,-8, NOTE_A5,4, NOTE_F5,8, NOTE_G5,8,
    REST,8, NOTE_E5,4,NOTE_C5,8, NOTE_D5,8, NOTE_B4,-4,
    NOTE_C5,-4, NOTE_G4,8, REST,4, NOTE_E4,-4, // repeats from 3
    NOTE_A4,4, NOTE_B4,4, NOTE_AS4,8, NOTE_A4,4,
    NOTE_G4,-8, NOTE_E5,-8, NOTE_G5,-8, NOTE_A5,4, NOTE_F5,8, NOTE_G5,8,
    REST,8, NOTE_E5,4,NOTE_C5,8, NOTE_D5,8, NOTE_B4,-4,
  
    
    REST,4, NOTE_G5,8, NOTE_FS5,8, NOTE_F5,8, NOTE_DS5,4, NOTE_E5,8,//7
    REST,8, NOTE_GS4,8, NOTE_A4,8, NOTE_C4,8, REST,8, NOTE_A4,8, NOTE_C5,8, NOTE_D5,8,
    REST,4, NOTE_DS5,4, REST,8, NOTE_D5,-4,
    NOTE_C5,2, REST,2,
  
    REST,4, NOTE_G5,8, NOTE_FS5,8, NOTE_F5,8, NOTE_DS5,4, NOTE_E5,8,//repeats from 7
    REST,8, NOTE_GS4,8, NOTE_A4,8, NOTE_C4,8, REST,8, NOTE_A4,8, NOTE_C5,8, NOTE_D5,8,
    REST,4, NOTE_DS5,4, REST,8, NOTE_D5,-4,
    NOTE_C5,2, REST,2,
  
    NOTE_C5,8, NOTE_C5,4, NOTE_C5,8, REST,8, NOTE_C5,8, NOTE_D5,4,//11
    NOTE_E5,8, NOTE_C5,4, NOTE_A4,8, NOTE_G4,2,
  
    NOTE_C5,8, NOTE_C5,4, NOTE_C5,8, REST,8, NOTE_C5,8, NOTE_D5,8, NOTE_E5,8,//13
    REST,1, 
    NOTE_C5,8, NOTE_C5,4, NOTE_C5,8, REST,8, NOTE_C5,8, NOTE_D5,4,
    NOTE_E5,8, NOTE_C5,4, NOTE_A4,8, NOTE_G4,2,
    NOTE_E5,8, NOTE_E5,8, REST,8, NOTE_E5,8, REST,8, NOTE_C5,8, NOTE_E5,4,
    NOTE_G5,4, REST,4, NOTE_G4,4, REST,4, 
    NOTE_C5,-4, NOTE_G4,8, REST,4, NOTE_E4,-4, // 19
    
    NOTE_A4,4, NOTE_B4,4, NOTE_AS4,8, NOTE_A4,4,
    NOTE_G4,-8, NOTE_E5,-8, NOTE_G5,-8, NOTE_A5,4, NOTE_F5,8, NOTE_G5,8,
    REST,8, NOTE_E5,4, NOTE_C5,8, NOTE_D5,8, NOTE_B4,-4,
  
    NOTE_C5,-4, NOTE_G4,8, REST,4, NOTE_E4,-4, // repeats from 19
    NOTE_A4,4, NOTE_B4,4, NOTE_AS4,8, NOTE_A4,4,
    NOTE_G4,-8, NOTE_E5,-8, NOTE_G5,-8, NOTE_A5,4, NOTE_F5,8, NOTE_G5,8,
    REST,8, NOTE_E5,4, NOTE_C5,8, NOTE_D5,8, NOTE_B4,-4,
  
    NOTE_E5,8, NOTE_C5,4, NOTE_G4,8, REST,4, NOTE_GS4,4,//23
    NOTE_A4,8, NOTE_F5,4, NOTE_F5,8, NOTE_A4,2,
    NOTE_D5,-8, NOTE_A5,-8, NOTE_A5,-8, NOTE_A5,-8, NOTE_G5,-8, NOTE_F5,-8,
    
    NOTE_E5,8, NOTE_C5,4, NOTE_A4,8, NOTE_G4,2, //26
    NOTE_E5,8, NOTE_C5,4, NOTE_G4,8, REST,4, NOTE_GS4,4,
    NOTE_A4,8, NOTE_F5,4, NOTE_F5,8, NOTE_A4,2,
    NOTE_B4,8, NOTE_F5,4, NOTE_F5,8, NOTE_F5,-8, NOTE_E5,-8, NOTE_D5,-8,
    NOTE_C5,8, NOTE_E4,4, NOTE_E4,8, NOTE_C4,2,
  
    NOTE_E5,8, NOTE_C5,4, NOTE_G4,8, REST,4, NOTE_GS4,4,//repeats from 23
    NOTE_A4,8, NOTE_F5,4, NOTE_F5,8, NOTE_A4,2,
    NOTE_D5,-8, NOTE_A5,-8, NOTE_A5,-8, NOTE_A5,-8, NOTE_G5,-8, NOTE_F5,-8,
    
    NOTE_E5,8, NOTE_C5,4, NOTE_A4,8, NOTE_G4,2, //26
    NOTE_E5,8, NOTE_C5,4, NOTE_G4,8, REST,4, NOTE_GS4,4,
    NOTE_A4,8, NOTE_F5,4, NOTE_F5,8, NOTE_A4,2,
    NOTE_B4,8, NOTE_F5,4, NOTE_F5,8, NOTE_F5,-8, NOTE_E5,-8, NOTE_D5,-8,
    NOTE_C5,8, NOTE_E4,4, NOTE_E4,8, NOTE_C4,2,
    NOTE_C5,8, NOTE_C5,4, NOTE_C5,8, REST,8, NOTE_C5,8, NOTE_D5,8, NOTE_E5,8,
    REST,1,
  
    NOTE_C5,8, NOTE_C5,4, NOTE_C5,8, REST,8, NOTE_C5,8, NOTE_D5,4, //33
    NOTE_E5,8, NOTE_C5,4, NOTE_A4,8, NOTE_G4,2,
    NOTE_E5,8, NOTE_E5,8, REST,8, NOTE_E5,8, REST,8, NOTE_C5,8, NOTE_E5,4,
    NOTE_G5,4, REST,4, NOTE_G4,4, REST,4, 
    NOTE_E5,8, NOTE_C5,4, NOTE_G4,8, REST,4, NOTE_GS4,4,
    NOTE_A4,8, NOTE_F5,4, NOTE_F5,8, NOTE_A4,2,
    NOTE_D5,-8, NOTE_A5,-8, NOTE_A5,-8, NOTE_A5,-8, NOTE_G5,-8, NOTE_F5,-8,
    
    NOTE_E5,8, NOTE_C5,4, NOTE_A4,8, NOTE_G4,2, //40
    NOTE_E5,8, NOTE_C5,4, NOTE_G4,8, REST,4, NOTE_GS4,4,
    NOTE_A4,8, NOTE_F5,4, NOTE_F5,8, NOTE_A4,2,
    NOTE_B4,8, NOTE_F5,4, NOTE_F5,8, NOTE_F5,-8, NOTE_E5,-8, NOTE_D5,-8,
    NOTE_C5,8, NOTE_E4,4, NOTE_E4,8, NOTE_C4,2,
    
    //game over sound
    NOTE_C5,-4, NOTE_G4,-4, NOTE_E4,4, //45
    NOTE_A4,-8, NOTE_B4,-8, NOTE_A4,-8, NOTE_GS4,-8, NOTE_AS4,-8, NOTE_GS4,-8,
    NOTE_G4,8, NOTE_D4,8, NOTE_E4,-2, 
];