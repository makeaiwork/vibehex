//! Strings of Labs › Screen vocabulary's AI cleanup row: the six columns of
//! [`super::super::TRANSLATIONS`] (Polish, Chinese (Simplified), Japanese,
//! German, Spanish, Ukrainian), then French, Brazilian Portuguese, and
//! Russian.

#[rustfmt::skip]
pub(super) const TRANSLATIONS: &[(&str, [&str; 6])] = &[
    ("Also give them to AI cleanup", ["Przekazuj je też do porządkowania przez AI", "也提供给 AI 整理", "AI による整えにも渡す", "Auch an die KI-Bereinigung geben", "Pasarlos también a la limpieza con IA", "Передавати їх і впорядкуванню ШІ"]),
    ("Cleanup can fix a name the speech model misheard. With a cloud engine these names are sent to it with the dictation.", ["Porządkowanie może poprawić nazwę, którą model mowy źle usłyszał. Z silnikiem w chmurze te nazwy trafiają do niego razem z dyktowaniem.", "整理可以修正语音模型听错的名称。使用云端引擎时，这些名称会随听写内容一起发送给它。", "音声モデルが聞き違えた名前を、整えで直せます。クラウドのエンジンでは、これらの名前が音声入力の内容と一緒に送られます。", "Die Bereinigung kann einen Namen korrigieren, den das Sprachmodell falsch verstanden hat. Mit einer Cloud-Engine werden diese Namen mit dem Diktat an sie gesendet.", "La limpieza puede corregir un nombre que el modelo de voz oyó mal. Con un motor en la nube, estos nombres se le envían junto con el dictado.", "Впорядкування може виправити назву, яку модель мовлення почула неправильно. З хмарним рушієм ці назви надсилаються йому разом із диктуванням."]),
];

#[rustfmt::skip]
pub(in super::super) const FRENCH: &[(&str, &str)] = &[
    ("Also give them to AI cleanup", "Les transmettre aussi au nettoyage par IA"),
    ("Cleanup can fix a name the speech model misheard. With a cloud engine these names are sent to it with the dictation.", "Le nettoyage peut corriger un nom que le modèle vocal a mal entendu. Avec un moteur dans le cloud, ces noms lui sont envoyés avec la dictée."),
];

#[rustfmt::skip]
pub(in super::super) const PORTUGUESE: &[(&str, &str)] = &[
    ("Also give them to AI cleanup", "Passá-los também para a limpeza com IA"),
    ("Cleanup can fix a name the speech model misheard. With a cloud engine these names are sent to it with the dictation.", "A limpeza pode corrigir um nome que o modelo de voz ouviu errado. Com um mecanismo na nuvem, esses nomes são enviados a ele junto com o ditado."),
];

#[rustfmt::skip]
pub(in super::super) const RUSSIAN: &[(&str, &str)] = &[
    ("Also give them to AI cleanup", "Передавать их и ИИ-очистке"),
    ("Cleanup can fix a name the speech model misheard. With a cloud engine these names are sent to it with the dictation.", "Очистка может исправить имя, которое модель речи расслышала неверно. С облачным движком эти имена отправляются ему вместе с диктовкой."),
];
