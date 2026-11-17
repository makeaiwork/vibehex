//! Strings of History's "What VibeHex read" section
//! (`rune_ui/history/read.rs`): the six columns of
//! [`super::super::TRANSLATIONS`] (Polish, Chinese (Simplified), Japanese,
//! German, Spanish, Ukrainian), then French, Brazilian Portuguese, and
//! Russian.

#[rustfmt::skip]
pub(super) const TRANSLATIONS: &[(&str, [&str; 6])] = &[
    ("What VibeHex read", ["Co odczytał VibeHex", "VibeHex 读取了什么", "VibeHex が読み取ったもの", "Was VibeHex gelesen hat", "Lo que leyó VibeHex", "Що прочитав VibeHex"]),
    ("Names from the window", ["Nazwy z okna", "窗口中的名称", "ウインドウの名前", "Namen aus dem Fenster", "Nombres de la ventana", "Назви з вікна"]),
    ("{}: given to AI cleanup", ["{}: przekazano do porządkowania przez AI", "{}：已提供给 AI 整理", "{}：AI による整えに渡しました", "{}: an die KI-Bereinigung gegeben", "{}: se pasó a la limpieza con IA", "{}: передано впорядкуванню ШІ"]),
    ("Marked names were also given to AI cleanup.", ["Zaznaczone nazwy przekazano też do porządkowania przez AI.", "标出的名称也提供给了 AI 整理。", "印の付いた名前は AI による整えにも渡しました。", "Markierte Namen wurden auch an die KI-Bereinigung gegeben.", "Los nombres marcados también se pasaron a la limpieza con IA.", "Позначені назви також передано впорядкуванню ШІ."]),
    ("AI cleanup was not given these names.", ["Tych nazw nie przekazano do porządkowania przez AI.", "这些名称没有提供给 AI 整理。", "これらの名前は AI による整えに渡していません。", "Diese Namen wurden nicht an die KI-Bereinigung gegeben.", "Estos nombres no se pasaron a la limpieza con IA.", "Ці назви не передано впорядкуванню ШІ."]),
    ("Text near the cursor", ["Tekst przy kursorze", "光标附近的文字", "カーソル付近のテキスト", "Text am Cursor", "Texto junto al cursor", "Текст біля курсора"]),
    ("Before the cursor", ["Przed kursorem", "光标之前", "カーソルの前", "Vor dem Cursor", "Antes del cursor", "Перед курсором"]),
    ("Selected", ["Zaznaczone", "已选中", "選択部分", "Ausgewählt", "Seleccionado", "Виділене"]),
    ("After the cursor", ["Za kursorem", "光标之后", "カーソルの後", "Nach dem Cursor", "Después del cursor", "Після курсора"]),
    ("AI cleanup was given all of it.", ["Porządkowanie przez AI otrzymało całość.", "AI 整理收到了全部内容。", "AI による整えにすべて渡しました。", "Die KI-Bereinigung hat alles davon bekommen.", "La limpieza con IA recibió todo.", "Впорядкування ШІ отримало все."]),
    ("The built-in model was given only the end of the text before the cursor and the selection.", ["Wbudowany model otrzymał tylko koniec tekstu przed kursorem i zaznaczenie.", "内置模型只收到了光标前文字的末尾和选中的部分。", "内蔵モデルには、カーソルの前のテキストの末尾と選択部分だけを渡しました。", "Das eingebaute Modell hat nur das Ende des Texts vor dem Cursor und die Auswahl bekommen.", "El modelo integrado solo recibió el final del texto antes del cursor y la selección.", "Вбудована модель отримала лише кінець тексту перед курсором і виділене."]),
    ("AI cleanup was not given this text.", ["Tego tekstu nie przekazano do porządkowania przez AI.", "这段文字没有提供给 AI 整理。", "このテキストは AI による整えに渡していません。", "Dieser Text wurde nicht an die KI-Bereinigung gegeben.", "Este texto no se pasó a la limpieza con IA.", "Цей текст не передано впорядкуванню ШІ."]),
    ("No names could be read from this window.", ["Z tego okna nie udało się odczytać żadnych nazw.", "无法从此窗口读取任何名称。", "このウインドウからは名前を読み取れませんでした。", "Aus diesem Fenster konnten keine Namen gelesen werden.", "No se pudo leer ningún nombre de esta ventana.", "З цього вікна не вдалося прочитати жодної назви."]),
    ("This app gave no text near the cursor.", ["Ta aplikacja nie podała tekstu przy kursorze.", "此应用没有提供光标附近的文字。", "このアプリはカーソル付近のテキストを渡しませんでした。", "Diese App hat keinen Text am Cursor geliefert.", "Esta app no dio texto junto al cursor.", "Ця програма не дала тексту біля курсора."]),
    ("AI cleanup repeated this text instead of the dictation, so it was asked again without it.", ["Porządkowanie przez AI powtórzyło ten tekst zamiast dyktowania, więc zapytano je ponownie bez niego.", "AI 整理重复了这段文字而不是听写内容，所以在去掉它之后重新询问了一次。", "AI による整えが音声入力の代わりにこのテキストを繰り返したため、このテキストなしでもう一度尋ねました。", "Die KI-Bereinigung hat diesen Text statt des Diktats wiederholt, daher wurde sie ohne ihn erneut gefragt.", "La limpieza con IA repitió este texto en lugar del dictado, así que se le volvió a preguntar sin él.", "Впорядкування ШІ повторило цей текст замість диктування, тож його запитали ще раз без нього."]),
];

#[rustfmt::skip]
pub(in super::super) const FRENCH: &[(&str, &str)] = &[
    ("What VibeHex read", "Ce que VibeHex a lu"),
    ("Names from the window", "Noms de la fenêtre"),
    ("{}: given to AI cleanup", "{} : transmis au nettoyage par IA"),
    ("Marked names were also given to AI cleanup.", "Les noms marqués ont aussi été transmis au nettoyage par IA."),
    ("AI cleanup was not given these names.", "Ces noms n'ont pas été transmis au nettoyage par IA."),
    ("Text near the cursor", "Texte près du curseur"),
    ("Before the cursor", "Avant le curseur"),
    ("Selected", "Sélection"),
    ("After the cursor", "Après le curseur"),
    ("AI cleanup was given all of it.", "Le nettoyage par IA l'a reçu en entier."),
    ("The built-in model was given only the end of the text before the cursor and the selection.", "Le modèle intégré n'a reçu que la fin du texte avant le curseur et la sélection."),
    ("AI cleanup was not given this text.", "Ce texte n'a pas été transmis au nettoyage par IA."),
    ("No names could be read from this window.", "Aucun nom n'a pu être lu dans cette fenêtre."),
    ("This app gave no text near the cursor.", "Cette app n'a donné aucun texte près du curseur."),
    ("AI cleanup repeated this text instead of the dictation, so it was asked again without it.", "Le nettoyage par IA a répété ce texte au lieu de la dictée, il a donc été interrogé de nouveau sans lui."),
];

#[rustfmt::skip]
pub(in super::super) const PORTUGUESE: &[(&str, &str)] = &[
    ("What VibeHex read", "O que o VibeHex leu"),
    ("Names from the window", "Nomes da janela"),
    ("{}: given to AI cleanup", "{}: passado para a limpeza com IA"),
    ("Marked names were also given to AI cleanup.", "Os nomes marcados também foram passados para a limpeza com IA."),
    ("AI cleanup was not given these names.", "Esses nomes não foram passados para a limpeza com IA."),
    ("Text near the cursor", "Texto perto do cursor"),
    ("Before the cursor", "Antes do cursor"),
    ("Selected", "Selecionado"),
    ("After the cursor", "Depois do cursor"),
    ("AI cleanup was given all of it.", "A limpeza com IA recebeu tudo."),
    ("The built-in model was given only the end of the text before the cursor and the selection.", "O modelo embutido recebeu só o fim do texto antes do cursor e a seleção."),
    ("AI cleanup was not given this text.", "Esse texto não foi passado para a limpeza com IA."),
    ("No names could be read from this window.", "Nenhum nome pôde ser lido desta janela."),
    ("This app gave no text near the cursor.", "Este app não deu nenhum texto perto do cursor."),
    ("AI cleanup repeated this text instead of the dictation, so it was asked again without it.", "A limpeza com IA repetiu esse texto em vez do ditado, então foi consultada de novo sem ele."),
];

#[rustfmt::skip]
pub(in super::super) const RUSSIAN: &[(&str, &str)] = &[
    ("What VibeHex read", "Что прочитал VibeHex"),
    ("Names from the window", "Имена из окна"),
    ("{}: given to AI cleanup", "{}: передано ИИ-очистке"),
    ("Marked names were also given to AI cleanup.", "Отмеченные имена переданы и ИИ-очистке."),
    ("AI cleanup was not given these names.", "Эти имена не передавались ИИ-очистке."),
    ("Text near the cursor", "Текст у курсора"),
    ("Before the cursor", "До курсора"),
    ("Selected", "Выделенное"),
    ("After the cursor", "После курсора"),
    ("AI cleanup was given all of it.", "ИИ-очистка получила его целиком."),
    ("The built-in model was given only the end of the text before the cursor and the selection.", "Встроенная модель получила только конец текста до курсора и выделенное."),
    ("AI cleanup was not given this text.", "Этот текст не передавался ИИ-очистке."),
    ("No names could be read from this window.", "Из этого окна не удалось прочитать ни одного имени."),
    ("This app gave no text near the cursor.", "Это приложение не дало текста у курсора."),
    ("AI cleanup repeated this text instead of the dictation, so it was asked again without it.", "ИИ-очистка повторила этот текст вместо диктовки, поэтому её спросили ещё раз без него."),
];
