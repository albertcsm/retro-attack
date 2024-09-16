import init, { Game } from './pkg/game_wasm.js';
import spaceshipImageSrc from './images/spaceship.png';
import rocketImageSrc from './images/rocket.png';

async function initGame() {
    await init();
    const canvas = document.getElementById("gameCanvas");
    const ctx = canvas.getContext("2d");
    const startButton = document.getElementById("startButton");

    let game = new Game();
    let gameStarted = false;
    let updateInterval;

    // Define tileSize
    const tileSize = 100; // You can adjust this value as needed

    // Load spaceship image
    const spaceshipImage = new Image();
    spaceshipImage.src = spaceshipImageSrc;

    // Load rocket image
    const rocketImage = new Image();
    rocketImage.src = rocketImageSrc;

    function renderMap() {
        const mapData = game.get_visible_map();
        const width = game.get_visible_width();
        const height = game.get_visible_height();

        // Set black background
        ctx.fillStyle = "#000";
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        for (let y = 0; y < height; y++) {
            for (let x = 0; x < width; x++) {
                const index = y * width + x;
                const tileType = mapData[index];

                let padding = tileSize * 0.05;
                ctx.lineWidth = 3;
                ctx.strokeStyle = tileType === 1 ? "#00FFFF" : "#080808";
                ctx.strokeRect(x * tileSize + padding, y * tileSize + padding, tileSize - 2 * padding, tileSize - 2 * padding);

                if (tileType === 2) {
                    renderRocket(x, y);
                }
            }
        }
    }

    function renderRocket(x, y) {
        if (rocketImage.complete) {
            ctx.drawImage(rocketImage, x * tileSize, y * tileSize, tileSize, tileSize);
        } else {
            // Fallback to a colored rectangle if the image hasn't loaded
            ctx.fillStyle = "yellow";
            ctx.fillRect(x * tileSize, y * tileSize, tileSize, tileSize);
        }
    }

    function renderPlayer() {
        const player = game.get_player(); // Get player coordinates from Rust
        if (spaceshipImage.complete) {
            ctx.drawImage(spaceshipImage, 0, player.y() * tileSize, tileSize, tileSize);
        } else {
            // Fallback to a colored rectangle if the image hasn't loaded
            ctx.fillStyle = "red";
            ctx.fillRect(0, player.y() * tileSize, tileSize, tileSize);
        }
    }

    function renderBullets() {
        const bullets = game.bullets();
        const bulletRadius = 5; // Radius of the bullet circle
        const scroll_x = game.get_visible_x(); // Get the current scroll_x value

        bullets.forEach(bullet => {
            ctx.fillStyle = 'cyan';
            // Calculate the center position of the tile, adjusting for scroll_x
            const centerX = (bullet.x - scroll_x) * tileSize + (tileSize / 2);
            const centerY = bullet.y * tileSize + (tileSize / 2);
            ctx.beginPath();
            ctx.arc(centerX, centerY, bulletRadius, 0, Math.PI * 2);
            ctx.fill();
        });
    }

    let keys = {
        "ArrowUp": 0,
        "ArrowDown": 0,
        " ": 0  // Add Space key
    };

    window.addEventListener("keydown", (e) => {
        if (e.key in keys && keys[e.key] === 0) {
            keys[e.key] = 1;
        }
    });

    window.addEventListener("keyup", (e) => {
        if (e.key in keys) {
            keys[e.key] = 0;
        }
    });

    function handleInput() {
        if (keys["ArrowUp"] === 1) {
            game.move_up();
            keys["ArrowUp"] = 2;
        }
        if (keys["ArrowDown"] === 1) {
            game.move_down();
            keys["ArrowDown"] = 2;
        }
        if (keys[" "] === 1) {
            game.fire();
            keys[" "] = 2;  // Prevent continuous firing while holding the key
        }
    }

    function gameLoop() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        renderMap();
        renderPlayer();
        renderBullets();
        if (gameStarted) {
            handleInput();
        }
        requestAnimationFrame(gameLoop);
    }

    function updateGame() {
        if (gameStarted) {
            game.update();
            // Check if the game has ended
            if (game.is_ended()) {
                gameStarted = false;
                startButton.disabled = false;
                startButton.textContent = "Restart";
                clearInterval(updateInterval); // Stop the update interval
                
                // Show game status
                const gameStatus = document.getElementById("gameStatus");
                if (game.is_won()) {
                    gameStatus.textContent = "You win!";
                    gameStatus.style.color = "green";
                } else {
                    gameStatus.textContent = "Game Over";
                    gameStatus.style.color = "red";
                }
            }
        }
    }

    startButton.addEventListener("click", () => {
        if (!gameStarted) {
            game = new Game(); // Reset the game
            game.start();
            gameStarted = true;
            updateInterval = setInterval(updateGame, 375);
            startButton.disabled = true;
            
            // Reset game status
            const gameStatus = document.getElementById("gameStatus");
            gameStatus.textContent = "";
            gameStatus.style.color = ""; // Reset color to default
        }
    });

    // Start rendering immediately
    gameLoop();
}

initGame();