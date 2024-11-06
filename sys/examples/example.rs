#![no_std]
#![no_main]

use core::{
    ffi::{c_char, c_int, c_uint},
    mem, ptr,
};

use sfml_sys::{
    sfBlack, sfClose, sfEvent, sfEvtClosed, sfFont_createFromFile, sfFont_destroy,
    sfMusic_createFromFile, sfMusic_destroy, sfMusic_play, sfRenderWindow_clear,
    sfRenderWindow_close, sfRenderWindow_create, sfRenderWindow_destroy, sfRenderWindow_display,
    sfRenderWindow_drawSprite, sfRenderWindow_drawText, sfRenderWindow_isOpen,
    sfRenderWindow_pollEvent, sfResize, sfSprite_create, sfSprite_destroy, sfSprite_setPosition,
    sfSprite_setTexture, sfText_create, sfText_destroy, sfText_setCharacterSize, sfText_setFont,
    sfText_setString, sfTexture_createFromFile, sfTexture_destroy, sfTrue, sfVector2f, sfVideoMode,
};

#[no_mangle]
unsafe extern "C" fn main(_: c_int, _: *const *const c_char) -> c_uint {
    // Create the main window
    let mode = sfVideoMode {
        bitsPerPixel: 32,
        width: 800,
        height: 600,
    };

    let window = sfRenderWindow_create(
        mode,
        c"SFML window".as_ptr(),
        sfResize.0 | sfClose.0,
        ptr::null(),
    );

    if window.is_null() {
        return 1;
    }

    // Load a sprite to display
    let texture = sfTexture_createFromFile(c"examples/sfml_logo.png".as_ptr(), ptr::null());

    if texture.is_null() {
        sfRenderWindow_destroy(window);
        return 1;
    }

    let sprite = sfSprite_create();
    sfSprite_setTexture(sprite, texture, sfTrue as i32);
    let sprite_position = sfVector2f { x: 200.0, y: 200.0 };
    sfSprite_setPosition(sprite, sprite_position);

    // Create a graphical text to display
    let font = sfFont_createFromFile(c"examples/tuffy.ttf".as_ptr());
    if font.is_null() {
        sfSprite_destroy(sprite);
        sfTexture_destroy(texture);
        sfRenderWindow_destroy(window);
        return 1;
    }

    let text = sfText_create();
    sfText_setString(text, c"Hello, SFML!".as_ptr());
    sfText_setFont(text, font);
    sfText_setCharacterSize(text, 50);

    // Load a music to play
    let music = sfMusic_createFromFile(c"examples/doodle_pop.ogg".as_ptr());
    if music.is_null() {
        sfText_destroy(text);
        sfFont_destroy(font);
        sfSprite_destroy(sprite);
        sfTexture_destroy(texture);
        sfRenderWindow_destroy(window);
        return 1;
    }

    // Play the music
    sfMusic_play(music);

    // Start the game loop
    let mut event: sfEvent = mem::zeroed();
    while sfRenderWindow_isOpen(window) != 0 {
        // Process events
        while sfRenderWindow_pollEvent(window, &raw mut event) != 0 {
            // Close window : exit
            if event.type_ == sfEvtClosed {
                sfRenderWindow_close(window);
            }

            // Clear the screen
            sfRenderWindow_clear(window, sfBlack);

            // Draw the sprite
            sfRenderWindow_drawSprite(window, sprite, ptr::null());

            // Draw the text
            sfRenderWindow_drawText(window, text, ptr::null());

            // Update the window
            sfRenderWindow_display(window);
        }
    }

    // Cleanup resources
    sfMusic_destroy(music);
    sfText_destroy(text);
    sfFont_destroy(font);
    sfSprite_destroy(sprite);
    sfTexture_destroy(texture);
    sfRenderWindow_destroy(window);

    0
}
