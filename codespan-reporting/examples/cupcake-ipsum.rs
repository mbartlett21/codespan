use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::{self, SimpleFile},
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};

fn main() -> Result<(), files::Error> {
    // This renders the example given in src/term/renderer.rs
    let file = SimpleFile::new(
        "test",
        r#"#!/bin/cupcake --ipsum
Cotton candy sesame snaps chocolate cake tart. Cupcake gummi bears
chupa chups jelly-o. Candy canes cotton candy liquorice danish jelly
beans biscuit macaroon. Gingerbread lemon drops donut gingerbread
halvah croissant biscuit donut. Marshmallow chocolate cotton candy.
Cake chocolate jelly beans sesame snaps tart caramels gingerbread.
Marzipan croissant sweet roll.

Cupcake ipsum dolor. Sit amet marshmallow topping cheesecake
muffin. Halvah croissant candy canes bonbon candy. Apple pie jelly
ice cream soufflé macaroon toffee marzipan ice cream. Lollipop bear
claw marzipan donut ice cream liquorice chocolate gummies. Sweet roll
pastry lollipop apple pie. Icing croissant tootsie roll sweet roll
icing danish. Danish lemon drops tart cheesecake. Marzipan toffee
pastry jujubes bonbon tiramisu chupa chups soufflé. Halvah macaroon
cupcake tart. Jelly beans icing cotton candy marshmallow. Donut
dessert sweet jelly beans toffee lollipop powder. Jujubes sugar plum
gingerbread dessert candy danish cookie fruitcake fruitcake. Sugar
plum croissant chocolate cake candy canes gingerbread ice cream.

Carrot cake bonbon cookie topping bonbon candy sesame snaps. Oat cake
croissant gummies bear claw. Jujubes toffee cake powder. Marzipan
biscuit donut. Sesame snaps danish cupcake sweet. Bear claw marzipan
bonbon jelly beans halvah sweet roll. Tootsie roll jelly-o bear claw
sweet gummies liquorice donut. Cotton candy bonbon carrot cake. Candy
canes soufflé tart chocolate bar jelly gingerbread muffin. Soufflé
chocolate gingerbread. Liquorice danish lollipop macaroon candy
chocolate cake cake. Brownie macaroon tiramisu apple pie powder tart.
Dragée macaroon jelly-o bear claw. Lollipop ice cream ice cream
marshmallow bear claw. Liquorice brownie apple pie lollipop lollipop
fruitcake pie chocolate bar muffin.

Muffin danish chocolate soufflé pastry icing bonbon oat cake.
Powder cake jujubes oat cake. Lemon drops tootsie roll marshmallow
lollipop candy canes oat cake toffee. Gingerbread brownie cake danish
pastry sesame snaps wafer jelly beans. Jujubes carrot cake jujubes.

Brownie lemon drops chocolate jelly-o candy canes. Danish marzipan
jujubes soufflé carrot cake marshmallow tiramisu caramels candy canes.
Fruitcake jelly-o danish toffee. Tootsie roll pastry cheesecake
soufflé marzipan. Chocolate bar oat cake jujubes lollipop pastry
cupcake. Candy canes cupcake toffee gingerbread candy canes muffin

Chocolate bar chocolate cake topping. Tiramisu muffin donut. Oat cake
bonbon powder donut danish bonbon powder. Sesame snaps apple pie tart.
Pie marshmallow icing chocolate jelly beans sugar plum croissant.
Croissant ice cream cookie tootsie roll jelly beans cupcake. Cookie
liquorice pudding sweet roll pudding danish jelly beans bonbon. Lemon
drops soufflé sweet croissant jelly beans donut bonbon caramels. Ice
cream apple pie sesame snaps bear claw halvah dessert. Donut
marshmallow tart jujubes sesame snaps danish ice cream halvah
cheesecake. Cake marzipan oat cake. Pie icing sweet roll toffee oat
cake sweet roll lollipop pastry. Lollipop powder candy brownie. Chupa
chups carrot cake danish oat cake cookie cake pastry.

Cotton candy chocolate bar halvah gummi bears dragée tart. Chupa chups
pie topping dessert gingerbread. Cake pie cake. Gummi bears apple pie
chocolate pudding wafer pudding. Biscuit icing fruitcake. Halvah candy
halvah cake sesame snaps marzipan donut. Halvah oat cake tiramisu
bonbon biscuit sweet bear claw jelly beans. Sugar plum jelly beans
chocolate cake tootsie roll tart dessert tart. Jelly jujubes pie candy
canes cupcake apple pie toffee brownie danish. Carrot cake danish
candy canes lollipop sweet. Ice cream cookie sugar plum chocolate.
Cupcake gummies macaroon jelly.

Brownie dragée dragée. Halvah pudding brownie caramels candy icing
danish tootsie roll. Chocolate carrot cake oat cake. Jujubes cupcake
wafer. Muffin cupcake cheesecake sweet roll biscuit sesame snaps
sesame snaps pie. Halvah marzipan croissant. Ice cream caramels
dragée. Icing pie sugar plum. Bear claw pudding fruitcake. Jelly-o
cupcake lollipop. Jelly marshmallow dessert oat cake liquorice
chocolate cake tart. Muffin dessert gummi bears chocolate bar.
Tootsie roll carrot cake candy canes marzipan candy canes sweet.
Carrot cake jujubes chocolate cake danish cake caramels.

Macaroon brownie sugar plum dragée jujubes muffin. Bonbon chocolate
carrot cake pie jelly halvah pie gummi bears candy canes. Toffee
caramels chocolate bar macaroon dragée cupcake liquorice donut icing.
Dessert powder icing pie. Halvah jelly-o marshmallow tart chocolate
cake. Liquorice muffin chocolate marzipan marshmallow jelly. Liquorice
marshmallow macaroon pudding topping cake. Bonbon pastry brownie
gingerbread toffee chupa chups chupa chups jelly-o cotton candy.
"#,
    );
    // Text courtesy of cupcakeipsum.com

    let diagnostic = Diagnostic::error()
        .with_message("oh noes, a cupcake has occurred!")
        .with_code("0001")
        .with_labels(vec![
            // 10:9
            Label::secondary((), 460..2390).with_message("blah blah"),
            // 9:1
            Label::secondary((), 391..1936).with_message("blah blah"),
            // 39:10
            Label::primary((), 2188..2208).with_message("blah blah"),
            // 39:10
            Label::secondary((), 2188..2208).with_message("note: this is a note"),
            // 39:30
            Label::secondary((), 2209..2229).with_message("blah blah"),
            // 42:30
            Label::primary((), 2411..2429).with_message("blah blah"),
            // 82:13
            Label::primary((), 4769..4775),
            // 82:44
            Label::secondary((), 4800..4807).with_message("blah blah"),
        ])
        .with_notes(vec![
            "blah blah".to_owned(),
            "blah blah blah\nblah blah".to_owned(),
            "blah blah blah\nblah blah".to_owned(),
        ]);

    let writer = StandardStream::stderr(ColorChoice::Always);
    let mut config = term::Config::default();
    config.start_context_lines = 0;

    term::emit(&mut writer.lock(), &config, &file, &diagnostic)?;

    Ok(())
}
