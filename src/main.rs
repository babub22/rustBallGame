use raylib::ffi::KeyboardKey::*;
use raylib::prelude::*;
use rand::Rng;
use std::mem;

static screenW:f32 = 1280.0;
static screenH:f32 = 720.0;
static defBallSpeed:f32 = 800.0;

static ballRadius:f32 = 15.0;
static ballRadiusX2:f32 = 225.0;


struct Vector2i{
    x:i32, y:i32
}

#[derive(Default)]
#[derive(Clone)]
#[derive(Copy)]
struct Block{
    r:Rectangle, h:i32
}

fn min(a:f32, b: f32) -> f32{
    if a>b {
	return b
    }
    
    return a;
}

fn max(a:f32, b: f32) -> f32{
    if a>b {
	return a
    }
    
    return b;
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(screenW as i32, screenH as i32)
        .title("ball game rust")
        .build();

    rl.set_target_fps(60);
    rl.hide_cursor();
    rl.disable_cursor();

    let platformW = 140.0;
    let platformH = 20.0;

    let mut playerPlatform = Rectangle{ x: platformW / 2.0, y: screenH - platformH
					, width: platformW, height: platformH };

    const blocksNumX: usize = 10;
    const blocksNumY: usize = 4;

    let mut blocksArr: [Block; blocksNumX * blocksNumY] = [Default::default(); blocksNumX * blocksNumY];
    let mut blocksCount: usize = blocksNumY * blocksNumX;

    initBlocks(&mut blocksArr, blocksNumX, blocksNumY);

    let mut ballSpeed:f32 = defBallSpeed;
    let mut ballPos = Vector2{ x: (playerPlatform.x + playerPlatform.width/2.0), y: (playerPlatform.y - ballRadius) };
    let mut ballDir = Vector2{ x: 0.0, y: -1.0 };

    let mut mousePos;
    let mut mouseXNDCDelta = 0.0;

    let mut isBallLaunched: bool = false;
    let mut gameIsOver: bool = false;
    let mut playerWon: bool = false;

    let mut lastDirTime: f64 = 0.0;//rl.get_time();
    let mut lastSpeedDecr: f64 = 0.0;//rl.get_time();

    let mut youCanChangeDir = false;

    let mut deltaTime = 0.0;

    let mut helpScreen = false;

    while !rl.window_should_close() {	
        let mut d = rl.begin_drawing(&thread);

	deltaTime = d.get_frame_time();

	if(helpScreen){
	    if d.is_key_pressed(KEY_F1){
		helpScreen = false;
	    }
	    
	    d.clear_background(Color::BLACK);

	    d.draw_text(
		"Controls:\n\n F1 - close this window\n\n Esc - close game\n\n W,A,S,D - change ball direction\n\n R - restart game \n\nRules: \n\n to win, you need to break all white blocks\n\n player can change the direction of the ball's flight using the keys W,A,S,D\n\n after ball launching, ball loses speed, in order to restore speed, ball needs to touch white platform \n at the bottom of the screen, if the speed of the ball is too low, player will lose\n\n if the ball touches the bottom edge of the screen, player will also lose",
		(120) as i32,
		(120) as i32,
		20,
		Color::WHITE,
	    );
		
	    continue;
	}
	 
	// logic
	if d.is_key_pressed(KEY_V){
	    blocksCount = 1;
	}
	
	if !gameIsOver || playerWon
	{
	    mousePos = d.get_mouse_position();
	    mouseXNDCDelta = (mousePos.x) - mouseXNDCDelta;

	    if d.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
		isBallLaunched = true;
	    }

	    // move player platform
	    {
		let mut newX = playerPlatform.x + mouseXNDCDelta;

		if newX < 0.0 {
		    newX = 0.0;
		}else if newX > (screenW - platformW) {
		    newX = screenW - platformW;
		}
		
		playerPlatform.x = newX;
		mouseXNDCDelta = mousePos.x;
	    }
	    
	    if isBallLaunched {
		if d.is_key_pressed(KEY_W){
		    ballDir.y = -1.0;
		}else if d.is_key_pressed(KEY_A){
		    ballDir.x = -1.0;
		}else if d.is_key_pressed(KEY_D){
		    ballDir.x = 1.0;
		}else if d.is_key_pressed(KEY_S){
		    ballDir.y = 1.0;
		}
		
		if ballPos.x - ballRadius <= 0.0 {
		    ballDir.x = 1.0;
		}else if ballPos.x + ballRadius >= screenW {
		    ballDir.x = -1.0;
		}

		if (ballPos.y - ballRadius) <= 0.0 {
		    ballDir.y = 1.0;
		}else if (ballPos.y + ballRadius) >= screenH {
		    if playerWon {
			ballDir.y = -ballDir.y;
		    }else{
			gameIsOver = true;
		    }
		}

		if ballSpeed < 0.3 {
		    gameIsOver = true;
		}
		
		if !gameIsOver {
		    if !playerWon {
			if d.get_time() - lastSpeedDecr > 0.15 {
			    ballSpeed -= 1150.0 * deltaTime;
			    lastSpeedDecr = d.get_time();
			}
		    }
		    
		    let ballNextPos = Vector2{x: ballPos.x + (ballSpeed * ballDir.x * deltaTime), y:ballPos.y + (ballSpeed * ballDir.y * deltaTime) };
		    let mut isYCol = false;
		    let mut isXCol = false;
		    
		    if ballPos.y < screenH / 2.0 {
			let mut i = 0;

			for n in 0..blocksCount {
			    if circleVsRect(ballNextPos,blocksArr[n].r) {
				if blocksArr[n].r.x > ballNextPos.x || (blocksArr[n].r.x + blocksArr[n].r.width) < ballNextPos.x {
				    isXCol = true;
				}

				isYCol = true;
				if blocksArr[n].h != 0 {
				    blocksArr[n].h -= 1;
				}else{
				    continue;
				}
			    }
			    
			    blocksArr[i] = blocksArr[n];
			    i+=1;
			}
			
			blocksCount = i;
			
			if blocksCount == 0 {
			    playerWon = true;
			    ballSpeed = defBallSpeed;
			}
		    }else if circleVsRect(ballNextPos,playerPlatform) {
			ballDir.x = (((ballNextPos.x - playerPlatform.x) / (playerPlatform.width / 3.0)) as u8 ) as f32 - 1.0;
			ballSpeed = defBallSpeed;
			ballDir.y = -1.0;
		    }

		    if isXCol{
			ballDir.x = -ballDir.x;		    
		    }

		    if isYCol{
			ballDir.y = -ballDir.y;
		    }
		    
		    ballPos.y += ballSpeed * ballDir.y * deltaTime;
		    ballPos.x += ballSpeed * ballDir.x * deltaTime;
		}
	    }else{
		if d.is_key_pressed(KEY_F1){
		    helpScreen = true;
		}
		
		ballPos = Vector2{ x: (playerPlatform.x + playerPlatform.width/2.0), y: (playerPlatform.y - ballRadius) };
	    }
	}

//	if gameIsOver || playerWon
	{
	    if d.is_key_pressed(KEY_R){
		ballPos = Vector2{ x: (playerPlatform.x + playerPlatform.width/2.0), y: (playerPlatform.y - ballRadius) };
		ballDir = Vector2{ x: 0.0, y: -1.0 };

		blocksCount = blocksNumY * blocksNumX;
		initBlocks(&mut blocksArr, blocksNumX, blocksNumY);

		ballSpeed = defBallSpeed;

		isBallLaunched = false;
		gameIsOver = false;
		playerWon = false;
	    }
	}

	// drawing
	{
            d.clear_background(Color::BLACK);

	    if gameIsOver {
		d.draw_text(
		    "Game is Over Press R to restart",
		    (screenW / 2.0) as i32 - (mem::size_of_val("Game is Over Press R to restart") * 5) as i32,
		    (screenH / 2.0) as i32,
		    20,
		    Color::WHITE,
		);
	    }else if playerWon {
		d.draw_text(
		    "You won Press R to restart",
		    (screenW / 2.0) as i32 - (mem::size_of_val("You won Press R to restart") * 5) as i32,
		    (screenH / 2.0) as i32,
		    20,
		    Color::WHITE,
		);
	    }else if !isBallLaunched {
		d.draw_text(
		    "Click to launch ball",
		    (screenW / 2.0) as i32 - (mem::size_of_val("Click to launch ball") * 5) as i32,
		    (screenH / 2.0) as i32,
		    20,
		    Color::WHITE,
		);
		d.draw_text(
		    "F1 help screen ",
		    (screenW / 2.0) as i32 - (mem::size_of_val("F1 help screen ") * 5) as i32,
		    (screenH / 2.0) as i32 + 40,
		    20,
		    Color::WHITE,
		);
	    }
	    
	    for n in 0..blocksCount {
		d.draw_rectangle_rec(blocksArr[n].r, Color::WHITE);
		if blocksArr[n].h != 0 {
		    d.draw_text(
			&blocksArr[n].h.to_string(),
			(blocksArr[n].r.x + (blocksArr[n].r.width/ 2.0)) as i32,
			(blocksArr[n].r.y + (blocksArr[n].r.height/ 2.0) - 8.0) as i32,
			20,
			Color::BLACK,
		    );
		}
	    }

	    d.draw_circle(ballPos.x as i32, ballPos.y as i32, ballRadius, Color::WHITE);
	    d.draw_rectangle_rec(playerPlatform, Color::WHITE);
	}
    }
}


fn initBlocks(arr:&mut [Block], xNum: usize, yNum:usize){
    let mut rng = rand::thread_rng();
    
    let platformW = 120.0;
    let platformH = 40.0;

    for n in 0..(xNum*yNum) {
	arr[n].r = Rectangle{ x: (n%xNum) as f32 * (platformW + 7.0) + 7.0,
			      y: (n/xNum) as f32 * (platformH + 7.0) + platformH + 7.0, width: platformW, height: platformH };
	arr[n].h = rng.gen_range(0..3);
    }
}

fn circleVsRect(c:Vector2, rec:Rectangle) -> bool {
    let closestX = max(rec.x, min(c.x , rec.x + rec.width));
    let closestY = max(rec.y, min(c.y, rec.y + rec.height));

    let distX = c.x - closestX;
    let distY = c.y - closestY;

    return ((distX * distX) + (distY * distY)) <= ballRadiusX2;
}
