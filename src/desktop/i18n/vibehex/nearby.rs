//! Strings of Labs › Rewrite reads nearby text's card: where the text goes
//! with the cleanup engine chosen now, and its consent for cloud engines.
//! The six columns of [`super::super::TRANSLATIONS`] (Polish, Chinese
//! (Simplified), Japanese, German, Spanish, Ukrainian), then French,
//! Brazilian Portuguese, and Russian.

#[rustfmt::skip]
pub(super) const TRANSLATIONS: &[(&str, [&str; 6])] = &[
    ("Password fields are never read.", ["Pola haseł nie są nigdy czytane.", "绝不会读取密码字段。", "パスワード欄は決して読み取りません。", "Passwortfelder werden nie gelesen.", "Nunca se leen los campos de contraseña.", "Поля паролів ніколи не читаються."]),
    ("Send it to cloud engines", ["Wysyłaj go do silników w chmurze", "发送给云端引擎", "クラウドのエンジンにも送る", "An Cloud-Engines senden", "Enviarlo a motores en la nube", "Надсилати його хмарним рушіям"]),
    ("Codex, OpenRouter, or a server that is not this Mac gets the text around your cursor with each dictation.", ["Codex, OpenRouter lub serwer spoza tego Maca dostaje tekst wokół kursora przy każdym dyktowaniu.", "Codex、OpenRouter 或不在这台 Mac 上的服务器会在每次听写时收到光标周围的文字。", "Codex、OpenRouter、またはこの Mac 以外のサーバーが、音声入力のたびにカーソル周辺のテキストを受け取ります。", "Codex, OpenRouter oder ein Server, der nicht dieser Mac ist, erhält bei jedem Diktat den Text um deinen Cursor.", "Codex, OpenRouter o un servidor que no sea este Mac recibe el texto alrededor del cursor con cada dictado.", "Codex, OpenRouter або сервер не на цьому Mac отримує текст навколо курсора з кожним диктуванням."]),
    ("With your cleanup engine now, the nearby text stays on this Mac.", ["Z obecnym silnikiem porządkowania tekst wokół kursora zostaje na tym Macu.", "使用你当前的整理引擎，光标附近的文字会留在这台 Mac 上。", "いまの整えのエンジンでは、カーソル周辺のテキストはこの Mac から出ません。", "Mit deiner aktuellen Bereinigungs-Engine bleibt der Text um den Cursor auf diesem Mac.", "Con tu motor de limpieza actual, el texto cercano se queda en este Mac.", "З вашим нинішнім рушієм упорядкування текст поруч із курсором лишається на цьому Mac."]),
    ("With your cleanup engine now, the nearby text goes to the mode's OpenCode model, which may be a cloud provider.", ["Z obecnym silnikiem porządkowania tekst wokół kursora trafia do modelu OpenCode danego trybu, który może być dostawcą chmurowym.", "使用你当前的整理引擎，光标附近的文字会发送到该模式的 OpenCode 模型，该模型可能是云端提供商。", "いまの整えのエンジンでは、カーソル周辺のテキストはモードの OpenCode モデルに送られます。クラウドの提供元の場合があります。", "Mit deiner aktuellen Bereinigungs-Engine geht der Text um den Cursor an das OpenCode-Modell des Modus, das ein Cloud-Anbieter sein kann.", "Con tu motor de limpieza actual, el texto cercano va al modelo OpenCode del modo, que puede ser un proveedor en la nube.", "З вашим нинішнім рушієм упорядкування текст поруч із курсором надходить до моделі OpenCode режиму, яка може бути хмарним провайдером."]),
    ("Nothing reads it now: AI cleanup is off.", ["Teraz nic go nie czyta: porządkowanie przez AI jest wyłączone.", "目前没有任何功能读取它：AI 整理已关闭。", "いまは何も読み取りません：AI による整えがオフです。", "Derzeit liest ihn nichts: Die KI-Bereinigung ist aus.", "Ahora nada lo lee: la limpieza con IA está desactivada.", "Зараз його ніщо не читає: упорядкування ШІ вимкнено."]),
    ("Nothing reads it now: your server has no address.", ["Teraz nic go nie czyta: twój serwer nie ma adresu.", "目前没有任何功能读取它：你的服务器没有设置地址。", "いまは何も読み取りません：サーバーのアドレスがありません。", "Derzeit liest ihn nichts: Dein Server hat keine Adresse.", "Ahora nada lo lee: tu servidor no tiene dirección.", "Зараз його ніщо не читає: ваш сервер не має адреси."]),
    ("With your cleanup engine now, the nearby text is sent to your server at {} with each dictation.", ["Z obecnym silnikiem porządkowania tekst wokół kursora jest wysyłany do twojego serwera pod adresem {} przy każdym dyktowaniu.", "使用你当前的整理引擎，每次听写时，光标附近的文字都会发送到你位于 {} 的服务器。", "いまの整えのエンジンでは、音声入力のたびにカーソル周辺のテキストが {} のサーバーに送られます。", "Mit deiner aktuellen Bereinigungs-Engine wird der Text um den Cursor bei jedem Diktat an deinen Server unter {} gesendet.", "Con tu motor de limpieza actual, el texto cercano se envía a tu servidor en {} con cada dictado.", "З вашим нинішнім рушієм упорядкування текст поруч із курсором надсилається на ваш сервер {} з кожним диктуванням."]),
    ("Not used now: your cleanup runs on your server at {}. Turn on Send it to cloud engines to use it there.", ["Teraz nieużywany: porządkowanie działa na twoim serwerze pod adresem {}. Włącz „Wysyłaj go do silników w chmurze”, aby go tam używać.", "目前未使用：你的整理在位于 {} 的服务器上运行。开启“发送给云端引擎”即可在那里使用。", "いまは使われていません：整えは {} のサーバーで動いています。そこで使うには「クラウドのエンジンにも送る」をオンにしてください。", "Derzeit nicht genutzt: Deine Bereinigung läuft auf deinem Server unter {}. Schalte „An Cloud-Engines senden“ ein, um ihn dort zu nutzen.", "Ahora no se usa: tu limpieza se ejecuta en tu servidor en {}. Activa «Enviarlo a motores en la nube» para usarlo allí.", "Зараз не використовується: упорядкування працює на вашому сервері {}. Увімкніть «Надсилати його хмарним рушіям», щоб використовувати його там."]),
    ("With your cleanup engine now, the nearby text is sent to {} with each dictation.", ["Z obecnym silnikiem porządkowania tekst wokół kursora jest wysyłany do {} przy każdym dyktowaniu.", "使用你当前的整理引擎，每次听写时，光标附近的文字都会发送到 {}。", "いまの整えのエンジンでは、音声入力のたびにカーソル周辺のテキストが {} に送られます。", "Mit deiner aktuellen Bereinigungs-Engine wird der Text um den Cursor bei jedem Diktat an {} gesendet.", "Con tu motor de limpieza actual, el texto cercano se envía a {} con cada dictado.", "З вашим нинішнім рушієм упорядкування текст поруч із курсором надсилається до {} з кожним диктуванням."]),
    ("Not used now: your cleanup runs on {}. Turn on Send it to cloud engines to use it there.", ["Teraz nieużywany: porządkowanie działa w {}. Włącz „Wysyłaj go do silników w chmurze”, aby go tam używać.", "目前未使用：你的整理在 {} 上运行。开启“发送给云端引擎”即可在那里使用。", "いまは使われていません：整えは {} で動いています。そこで使うには「クラウドのエンジンにも送る」をオンにしてください。", "Derzeit nicht genutzt: Deine Bereinigung läuft auf {}. Schalte „An Cloud-Engines senden“ ein, um ihn dort zu nutzen.", "Ahora no se usa: tu limpieza se ejecuta en {}. Activa «Enviarlo a motores en la nube» para usarlo allí.", "Зараз не використовується: упорядкування працює на {}. Увімкніть «Надсилати його хмарним рушіям», щоб використовувати його там."]),
];

#[rustfmt::skip]
pub(in super::super) const FRENCH: &[(&str, &str)] = &[
    ("Password fields are never read.", "Les champs de mot de passe ne sont jamais lus."),
    ("Send it to cloud engines", "L'envoyer aux moteurs dans le cloud"),
    ("Codex, OpenRouter, or a server that is not this Mac gets the text around your cursor with each dictation.", "Codex, OpenRouter ou un serveur qui n'est pas ce Mac reçoit le texte autour de votre curseur à chaque dictée."),
    ("With your cleanup engine now, the nearby text stays on this Mac.", "Avec votre moteur de nettoyage actuel, le texte proche reste sur ce Mac."),
    ("With your cleanup engine now, the nearby text goes to the mode's OpenCode model, which may be a cloud provider.", "Avec votre moteur de nettoyage actuel, le texte proche va au modèle OpenCode du mode, qui peut être un fournisseur cloud."),
    ("Nothing reads it now: AI cleanup is off.", "Rien ne le lit pour l'instant : le nettoyage par IA est désactivé."),
    ("Nothing reads it now: your server has no address.", "Rien ne le lit pour l'instant : votre serveur n'a pas d'adresse."),
    ("With your cleanup engine now, the nearby text is sent to your server at {} with each dictation.", "Avec votre moteur de nettoyage actuel, le texte proche est envoyé à votre serveur {} à chaque dictée."),
    ("Not used now: your cleanup runs on your server at {}. Turn on Send it to cloud engines to use it there.", "Pas utilisé pour l'instant : votre nettoyage s'exécute sur votre serveur {}. Activez « L'envoyer aux moteurs dans le cloud » pour l'utiliser là-bas."),
    ("With your cleanup engine now, the nearby text is sent to {} with each dictation.", "Avec votre moteur de nettoyage actuel, le texte proche est envoyé à {} à chaque dictée."),
    ("Not used now: your cleanup runs on {}. Turn on Send it to cloud engines to use it there.", "Pas utilisé pour l'instant : votre nettoyage s'exécute sur {}. Activez « L'envoyer aux moteurs dans le cloud » pour l'utiliser là-bas."),
];

#[rustfmt::skip]
pub(in super::super) const PORTUGUESE: &[(&str, &str)] = &[
    ("Password fields are never read.", "Campos de senha nunca são lidos."),
    ("Send it to cloud engines", "Enviá-lo a mecanismos na nuvem"),
    ("Codex, OpenRouter, or a server that is not this Mac gets the text around your cursor with each dictation.", "Codex, OpenRouter ou um servidor que não seja este Mac recebe o texto ao redor do cursor a cada ditado."),
    ("With your cleanup engine now, the nearby text stays on this Mac.", "Com o seu mecanismo de limpeza atual, o texto próximo fica neste Mac."),
    ("With your cleanup engine now, the nearby text goes to the mode's OpenCode model, which may be a cloud provider.", "Com o seu mecanismo de limpeza atual, o texto próximo vai para o modelo OpenCode do modo, que pode ser um provedor na nuvem."),
    ("Nothing reads it now: AI cleanup is off.", "Nada o lê agora: a limpeza com IA está desligada."),
    ("Nothing reads it now: your server has no address.", "Nada o lê agora: seu servidor não tem endereço."),
    ("With your cleanup engine now, the nearby text is sent to your server at {} with each dictation.", "Com o seu mecanismo de limpeza atual, o texto próximo é enviado ao seu servidor em {} a cada ditado."),
    ("Not used now: your cleanup runs on your server at {}. Turn on Send it to cloud engines to use it there.", "Não é usado agora: sua limpeza roda no seu servidor em {}. Ative “Enviá-lo a mecanismos na nuvem” para usá-lo lá."),
    ("With your cleanup engine now, the nearby text is sent to {} with each dictation.", "Com o seu mecanismo de limpeza atual, o texto próximo é enviado ao {} a cada ditado."),
    ("Not used now: your cleanup runs on {}. Turn on Send it to cloud engines to use it there.", "Não é usado agora: sua limpeza roda no {}. Ative “Enviá-lo a mecanismos na nuvem” para usá-lo lá."),
];

#[rustfmt::skip]
pub(in super::super) const RUSSIAN: &[(&str, &str)] = &[
    ("Password fields are never read.", "Поля паролей никогда не читаются."),
    ("Send it to cloud engines", "Отправлять его облачным движкам"),
    ("Codex, OpenRouter, or a server that is not this Mac gets the text around your cursor with each dictation.", "Codex, OpenRouter или сервер не на этом Mac получает текст вокруг курсора с каждой диктовкой."),
    ("With your cleanup engine now, the nearby text stays on this Mac.", "С вашим нынешним движком очистки текст рядом с курсором остаётся на этом Mac."),
    ("With your cleanup engine now, the nearby text goes to the mode's OpenCode model, which may be a cloud provider.", "С вашим нынешним движком очистки текст рядом с курсором отправляется модели OpenCode режима, которая может быть облачным провайдером."),
    ("Nothing reads it now: AI cleanup is off.", "Сейчас его ничто не читает: ИИ-очистка выключена."),
    ("Nothing reads it now: your server has no address.", "Сейчас его ничто не читает: у вашего сервера нет адреса."),
    ("With your cleanup engine now, the nearby text is sent to your server at {} with each dictation.", "С вашим нынешним движком очистки текст рядом с курсором отправляется на ваш сервер {} с каждой диктовкой."),
    ("Not used now: your cleanup runs on your server at {}. Turn on Send it to cloud engines to use it there.", "Сейчас не используется: очистка работает на вашем сервере {}. Включите «Отправлять его облачным движкам», чтобы использовать его там."),
    ("With your cleanup engine now, the nearby text is sent to {} with each dictation.", "С вашим нынешним движком очистки текст рядом с курсором отправляется в {} с каждой диктовкой."),
    ("Not used now: your cleanup runs on {}. Turn on Send it to cloud engines to use it there.", "Сейчас не используется: очистка работает на {}. Включите «Отправлять его облачным движкам», чтобы использовать его там."),
];
