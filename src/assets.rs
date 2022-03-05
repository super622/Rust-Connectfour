use ggez::graphics;
use ggez::audio;
use ggez::Context;
use ggez::GameResult;

pub struct Assets {
    pub front: graphics::Image,
    pub back: graphics::Image,
    pub green: graphics::Image,
    pub green_alternative: graphics::Image,
    pub red: graphics::Image,
    pub red_alternative: graphics::Image,
    pub fall_sound: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let front = graphics::Image::new(ctx, "/front.png")?;
        let back = graphics::Image::new(ctx, "/back.png")?;
        let green = graphics::Image::new(ctx, "/green.png")?;
        let green_alternative = graphics::Image::new(ctx, "/green_question_trans.png")?;
        let red = graphics::Image::new(ctx, "/red.png")?;
        let red_alternative = graphics::Image::new(ctx, "/red_question_trans.png")?;
        let fall_sound = audio::Source::new(ctx, "/clank.wav")?;
        Ok(Assets {
            front,
            back,
            green,
            green_alternative
            ,red
            ,red_alternative
            ,fall_sound
        })
    }
}