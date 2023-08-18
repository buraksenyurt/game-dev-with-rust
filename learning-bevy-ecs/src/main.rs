use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup,setup_system).run()
}
pub fn setup_system(mut commands:Commands){
    // Commands nesnesini entity oluşturmak, yok etmek
    // entity'lere component eklemek ve çıkarmak için kullanırız.

    commands.spawn(Commando{
        name:"Rambo".to_string()
    });

    commands.spawn(Commando{
        name:"Lincoln Eco Six".to_string()
    });

    commands.spawn(Commando{
        name:"Bevy Soldier".to_string()
    });
}

#[derive(Component)]
pub struct Commando{
    pub name:String
}
