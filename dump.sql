PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL,
    checksum BLOB NOT NULL,
    execution_time BIGINT NOT NULL
);
INSERT INTO _sqlx_migrations VALUES(1,'create','2025-06-03 06:07:00',1,X'299b4ab07c9058ff85e1a08aeb8fe7e7aed39e42bbcdf01680bfb4b94abbe633e322663c570ff3d0745d37531884fac9',968750);
INSERT INTO _sqlx_migrations VALUES(2,'full joke','2025-06-03 06:07:00',1,X'6add1a5adfbdee9b3ab4013c103c5df94eea4b6032857362bc6e8f3ab7422ef60b6d94b796f948987bc546bcd103d3e8',1344958);
CREATE TABLE IF NOT EXISTS "jokes_v1" (
  whos_there VARCHAR(200) NOT NULL,
  answer_who VARCHAR(200) NOT NULL
);
CREATE TABLE jokes (
  id VARCHAR(200) UNIQUE PRIMARY KEY NOT NULL,
  whos_there VARCHAR(200) NOT NULL,
  answer_who VARCHAR(200) NOT NULL,
  joke_source VARCHAR(200) NOT NULL
);
INSERT INTO jokes VALUES('doris','Doris','Doris locked. Open up, please!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('kent','Kent','Kent you tell who I am by my voice?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('bacon','Bacon','Bacon a cake for your birthday.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('lena','Lena','Lena little closer and I’ll tell you!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('kiwi','Kiwi','Kiwi go to the store?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('alex-here','Alex','Hey, Alex the questions around here!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('troy','Troy','Troy ringing the doorbell!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('armageddon','Armageddon','Armageddon a little bored. Let’s go out.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('nobel','Nobel','No bell, that’s why I knocked!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('andrew','Andrew','Andrew a picture!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('etch','Etch','Bless you, friend.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('olive-you','Olive','Olive right next door to you.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('leash','Leash','Leash you could do is answer the doorbell!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('interrupting-sloth','Interrupting sloth','(wait for 10-20 seconds) Sloooooooooth','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('owl','Owl','Owl you doing today?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('sparrow','Sparrow','Sparrow no expense for a laugh!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('frank','Frank','Frank you for being my friend.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('broken-pencil','Broken pencil','Never mind, it’s pointless!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('from','From','Grammatically speaking, you should say “from whom.”','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('annie-line','Annie','Annie thing you can do I can better! (bonus points if you sing this line)','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('justin-hug','Justin','Justin time for a big hug!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('candice','Candice','Candice joke possibly get any worse?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('turtle','Turtle','Turtle-y awesome animal jokes!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('anita','Anita','Anita to borrow a pencil!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('cargo','Cargo','Cargo beep beep!','Chris Sloggett');
INSERT INTO jokes VALUES('dwayne','Dwayne','Dwayne the bathtub! I''m dwowning!','Bart Massey');
INSERT INTO jokes VALUES('defense','Defense','Defense has a hole in it—so our dog got loose.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('dozen','Dozen','Dozen anyone want to let me in?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('radio','Radio','Radio not, here I come!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('venice','Venice','Venice your mom coming home?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('some-bunny','Some bunny','Some bunny has been eating all my carrots!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('comb','Comb','Comb on down, and I’ll tell you!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('canoe','Canoe','Canoe come out and play today?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('bear','Bear','Bear with me; I’m trying to tell a joke!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('donut','Donut','Donut ask, it’s a secret!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('ice-cream-soda','Ice cream soda','Ice cream soda people can hear me!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('honey-bee','Honey bee','Honey bee a dear and get me some water.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('turnip','Turnip','Turnip the volume! It’s quiet in here.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('cash','Cash','No thanks, but I’ll take a peanut.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('mikey','Mikey','Mikey doesn’t fit in the keyhole!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('bed','Bed','Bed you can’t guess who I am!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('owls-say','Owls say','Yes, they do.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('ken-house','Ken','Ken I bring my dog to your house?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('abe','Abe','Abe C D E F G H…','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('burglar','Burglar','Burglars don’t knock!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('europe-poo','Europe','No, you’re a poo!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('tabby','Tabby','Tabby honest, these jokes are the cat’s meow!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('claire','Claire','Claire the way; I’m coming in!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('avenue','Avenue','Avenue knocked on this door before?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('lettuce','Lettuce','Let us in; we’re freezing!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('water','Water','Water you doing in my house?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('juneau','Juneau','Juneau the capital of Alaska?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('orange-door','Orange','Orange you going to answer the door?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('howard','Howard','Howard I know?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('oslo','Oslo','Oslo down, what’s the hurry!?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('ida','Ida','Surely you know it’s pronounced IdaHO.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('paws','Paws','Paws for effect, I’ve got more cat jokes!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('zoom','Zoom','Zoom did you think it was?','unknown');
INSERT INTO jokes VALUES('howl','Howl','Howl you know it’s me unless you open the door?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('luke','Luke','Luke through the keyhole to see!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('spell','Spell','W-H-O!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('amanda','Amanda','A man da fix your doorbell!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('amish','Amish','Awe, I miss you too.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('mustache','Mustache','I mustache you a question, but I’ll shave it for later.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('aristophanes','Aristophanes','… … uh, this knock-knock thing is hard. Ask me another?','Bart Massey');
INSERT INTO jokes VALUES('pudding','Pudding','Pudding on a few pounds from all these sweets!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('ada','Ada','Ada burger for lunch!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('beef','Beef','Before I get cold, you’d better let me in!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('al','Al','Al give you a high five if you open this door!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('kenya','Kenya','Kenya feel the love tonight? (sing along!)','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('banana','Banana','Banana split!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('orange-me','Orange','Orange you glad to see me?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('ice-cream','Ice cream','Ice cream if you don’t let me inside!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('billy-bob-joe-penny','Billy Bob Joe Penny','Exactly how many Billy Bob Joe Pennies do you know?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('witches','Witches','Witches the way to the store?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('dewey','Dewey','Dewey have to go to school today?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('fish','Fish','Fish you a happy day!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('alpaca','Alpaca','Alpaca the trunk, you a-pack-a the suitcase!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('monkey','Monkey','Monkey see. Monkey do.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('voodoo','Voodoo','Voodoo you think you are, asking all these questions!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('figs','Figs','Figs the doorbell, it’s broken!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('ketchup','Ketchup','Ketchup with me and I’ll tell you!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('barbie','Barbie','Barbie Q Chicken!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('beets','Beets','Beets me!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('yukon','Yukon','Yukon say that again!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('leaf','Leaf','Leaf me alone!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('police-here','Police','Police hurry—I’m freezing out here!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('alien','Alien','Just how many aliens do you know?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('tank','Tank','You’re welcome!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('broccoli','Broccoli','Broccoli doesn’t have a last name, silly.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('justin-by','Justin','Justin the neighborhood and thought I’d stop by.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('otto','Otto','Otto know what’s taking you so long!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('boo','Boo','You don''t have to cry about it!','Bart Massey');
INSERT INTO jokes VALUES('havana','Havana','Havana a wonderful time wish you were here!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('cow-go','Cow-go','No, cow go MOO!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('sue','Sue','Sue-prize! Happy birthday!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('needle','Needle','Needle little help gettin’ in the door.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('rough','Rough','Rough, rough! It’s your dog!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('will','Will','Will you let me in? It’s freezing out here!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('abbey','Abbey','Abbey birthday to you, Abbey birthday to you…','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('ben','Ben','Ben knocking for 20 minutes already.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('atch','Atch','Bless you!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('eileen','Eileen','Eileen over to pet the dog.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('a-herd','A herd','A herd you were home. Can you play?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('joe','Joe','Joking around with you is one of my favorite things to do!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('sherlock','Sherlock','Sherlock your door shut tight.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('tyrone','Tyrone','Tyrone shoelaces!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('who','Who','Is there an owl in here?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('alligator','Alligator','Alligator her a nice birthday present.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('doctor','Doctor','You’ve seen that TV show?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('scold','Scold','Scold outside—let me in!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('frog','Frog','Frog-et about your worries and enjoy the jokes!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('europe-not','Europe','No I’m not!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('chick','Chick','Chick your stove, I can smell it burning!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('ya','Ya','Aw, you’re so excited to see me!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('annie-door','Annie','Annie body going to open this door?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('cheese','Cheese','Cheese a nice girl.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('ken-here','Ken','Ken I come in? It’s freezing out here!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('feline','Feline','Feline fine, how about you?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('dishes','Dishes','Dishes a nice place you got here.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('butter','Butter','Butter be quick. I have to go to the bathroom!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('snow','Snow','Snow laughing matter.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('isabel','Isabel','Isabel working? I had to knock.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('robin','Robin','Robin your house!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('bee','Bee','Bee-hind you!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('cd','CD','CD person on your doorstep?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('adore','Adore','Adore is between us. Open up!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('nana','Nana','Nana your business who’s there.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('caesar','Caesar','Caesar quick, she’s running away.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('alex-door','Alex','Alex-plain when you open the door!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('amy','Amy','Amy fraid I’ve forgotten!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('nun','Nun','Nun of your business!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('pie','Pie','Pie want a piece of that delicious dessert!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('little-old-lady','Little old lady','I didn’t know you could yodel.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('amarillo','Amarillo','Amarillo nice guy!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('hatch','Hatch','Bless you, and cover your mouth next time!','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('wendy','Wendy','Wendy bell works again, I won’t have to knock.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('police-in','Police','Police (please) may I come in?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('olive-too','Olive','Olive you. Do you love me too?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('quiche','Quiche','Can I have a hug and a quiche?','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('goat','Goat','Goat to the door and find out.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
INSERT INTO jokes VALUES('alma','Alma','Alma not going to say.','https://lajollamom.com/kid-friendly-knock-knock-jokes');
CREATE TABLE recipes (
    id INTEGER PRIMARY KEY,
    whos_there TEXT NOT NULL,
    answer_who TEXT NOT NULL,
    recipe_source TEXT
);
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    recipe_id INTEGER NOT NULL,
    tag TEXT NOT NULL,
    FOREIGN KEY (recipe_id) REFERENCES recipes(id)
);
DELETE FROM sqlite_sequence;
COMMIT;
