use crate::MainState;
use ggez::GameResult;
type Vector2 = glam::Vec2;

// Kayalar ile oyuncunun ve kayalara gelen atışların çarpışma hallerini ele ala fonksiyon
pub fn handle_collisions(main_state: &mut MainState) -> GameResult {
    // State'teki her bir kaya dolaşılır
    for rock in &mut main_state.rocks {
        // Kaya ile oyuncu arasındaki mesafe hesaplarnır
        let player_distance = Vector2::new(
            rock.position.x - main_state.player.position.x,
            rock.position.y - main_state.player.position.y,
        );

        // Çarpışma varsa oyuncunun canı sıfırlanır
        if player_distance.length() < (main_state.player.size + rock.size) {
            println!(
                "Oyuncu ve kaya arasındaki mesafe {:?}",
                player_distance.length()
            );
            main_state.player.life = 0.;
        }
        // Şimdi state üstündeki her bir atışa bakılır
        for shot in &mut main_state.shots {
            let shot_distance = Vector2::new(
                shot.position.x - rock.position.x,
                shot.position.y - rock.position.y,
            );
            // Eğer atış ile kaya arası mesafe sıfırın altıysa
            if shot_distance.length() < (shot.size + rock.size) {
                // Atışın yok edilmesi için
                shot.life = 0.;
                // Kayanın yok edilmesi için
                rock.life = 0.;
                // Skore bir artar
                main_state.score += 1;
            }
        }
    }
    Ok(())
}
