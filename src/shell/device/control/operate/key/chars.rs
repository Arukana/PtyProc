use libc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Char {
    /// U+{0000..007F}: Basic Latin.
    BasicLatin([libc::c_uchar; 1]),
    /// U+{0080..00FF}: Latin-1 Supplement.
    Latin1Supplement([libc::c_uchar; 2]),
    /// U+{0100..017F}: Latin Extended-A.
    LatinExtendedA([libc::c_uchar; 2]),
    /// U+{0180..024F}: Latin Extended-B.
    LatinExtendedB([libc::c_uchar; 2]),
    /// U+{0250..02AF}: IPA Extensions.
    IpaExtensions([libc::c_uchar; 2]),
    /// U+{02B0..02FF}: Spacing Modifier Letters.
    SpacingModifierLetters([libc::c_uchar; 2]),
    /// U+{0300..036F}: Combining Diacritical Marks.
    CombiningDiacriticalMarks([libc::c_uchar; 2]),
    /// U+{0370..03FF}: Greek and Coptic.
    GreekandCoptic([libc::c_uchar; 2]),
    /// U+{0400..04FF}: Cyrillic.
    Cyrillic([libc::c_uchar; 2]),
    /// U+{0500..052F}: Cyrillic Supplement.
    CyrillicSupplement([libc::c_uchar; 2]),
    /// U+{0530..058F}: Armenian.
    Armenian([libc::c_uchar; 2]),
    /// U+{0590..05FF}: Hebrew.
    Hebrew([libc::c_uchar; 2]),
    /// U+{0600..06FF}: Arabic.
    Arabic([libc::c_uchar; 2]),
    /// U+{0700..074F}: Syriac.
    Syriac([libc::c_uchar; 2]),
    /// U+{0750..077F}: Arabic Supplement.
    ArabicSupplement([libc::c_uchar; 2]),
    /// U+{0780..07BF}: Thaana.
    Thaana([libc::c_uchar; 2]),
    /// U+{07C0..07FF}: NKo.
    NKo([libc::c_uchar; 2]),
    /// U+{0800..083F}: Samaritan.
    Samaritan([libc::c_uchar; 3]),
    /// U+{0840..085F}: Mandaic.
    Mandaic([libc::c_uchar; 3]),
    /// U+{08A0..08FF}: Arabic Extended-A.
    ArabicExtendedA([libc::c_uchar; 3]),
    /// U+{0900..097F}: Devanagari.
    Devanagari([libc::c_uchar; 3]),
    /// U+{0980..09FF}: Bengali.
    Bengali([libc::c_uchar; 3]),
    /// U+{0A00..0A7F}: Gurmukhi.
    Gurmukhi([libc::c_uchar; 3]),
    /// U+{0A80..0AFF}: Gujarati.
    Gujarati([libc::c_uchar; 3]),
    /// U+{0B00..0B7F}: Oriya.
    Oriya([libc::c_uchar; 3]),
    /// U+{0B80..0BFF}: Tamil.
    Tamil([libc::c_uchar; 3]),
    /// U+{0C00..0C7F}: Telugu.
    Telugu([libc::c_uchar; 3]),
    /// U+{0C80..0CFF}: Kannada.
    Kannada([libc::c_uchar; 3]),
    /// U+{0D00..0D7F}: Malayalam.
    Malayalam([libc::c_uchar; 3]),
    /// U+{0D80..0DFF}: Sinhala.
    Sinhala([libc::c_uchar; 3]),
    /// U+{0E00..0E7F}: Thai.
    Thai([libc::c_uchar; 3]),
    /// U+{0E80..0EFF}: Lao.
    Lao([libc::c_uchar; 3]),
    /// U+{0F00..0FFF}: Tibetan.
    Tibetan([libc::c_uchar; 3]),
    /// U+{1000..109F}: Myanmar.
    Myanmar([libc::c_uchar; 3]),
    /// U+{10A0..10FF}: Georgian.
    Georgian([libc::c_uchar; 3]),
    /// U+{1100..11FF}: Hangul Jamo.
    HangulJamo([libc::c_uchar; 3]),
    /// U+{1200..137F}: Ethiopic.
    Ethiopic([libc::c_uchar; 3]),
    /// U+{1380..139F}: Ethiopic Supplement.
    EthiopicSupplement([libc::c_uchar; 3]),
    /// U+{13A0..13FF}: Cherokee.
    Cherokee([libc::c_uchar; 3]),
    /// U+{1400..167F}: Unified Canadian Aboriginal Syllabics.
    UnifiedCanadianAboriginalSyllabics([libc::c_uchar; 3]),
    /// U+{1680..169F}: Ogham.
    Ogham([libc::c_uchar; 3]),
    /// U+{16A0..16FF}: Runic.
    Runic([libc::c_uchar; 3]),
    /// U+{1700..171F}: Tagalog.
    Tagalog([libc::c_uchar; 3]),
    /// U+{1720..173F}: Hanunoo.
    Hanunoo([libc::c_uchar; 3]),
    /// U+{1740..175F}: Buhid.
    Buhid([libc::c_uchar; 3]),
    /// U+{1760..177F}: Tagbanwa.
    Tagbanwa([libc::c_uchar; 3]),
    /// U+{1780..17FF}: Khmer.
    Khmer([libc::c_uchar; 3]),
    /// U+{1800..18AF}: Mongolian.
    Mongolian([libc::c_uchar; 3]),
    /// U+{18B0..18FF}: Unified Canadian Aboriginal Syllabics Extended.
    UnifiedCanadianAboriginalSyllabicsExtended([libc::c_uchar; 3]),
    /// U+{1900..194F}: Limbu.
    Limbu([libc::c_uchar; 3]),
    /// U+{1950..197F}: Tai Le.
    TaiLe([libc::c_uchar; 3]),
    /// U+{1980..19DF}: New Tai Lue.
    NewTaiLue([libc::c_uchar; 3]),
    /// U+{19E0..19FF}: Khmer Symbols.
    KhmerSymbols([libc::c_uchar; 3]),
    /// U+{1A00..1A1F}: Buginese.
    Buginese([libc::c_uchar; 3]),
    /// U+{1A20..1AAF}: Tai Tham.
    TaiTham([libc::c_uchar; 3]),
    /// U+{1AB0..1AFF}: Combining Diacritical Marks Extended.
    CombiningDiacriticalMarksExtended([libc::c_uchar; 3]),
    /// U+{1B00..1B7F}: Balinese.
    Balinese([libc::c_uchar; 3]),
    /// U+{1B80..1BBF}: Sundanese.
    Sundanese([libc::c_uchar; 3]),
    /// U+{1BC0..1BFF}: Batak.
    Batak([libc::c_uchar; 3]),
    /// U+{1C00..1C4F}: Lepcha.
    Lepcha([libc::c_uchar; 3]),
    /// U+{1C50..1C7F}: Ol Chiki.
    OlChiki([libc::c_uchar; 3]),
    /// U+{1CC0..1CCF}: Sundanese Supplement.
    SundaneseSupplement([libc::c_uchar; 3]),
    /// U+{1CD0..1CFF}: Vedic Extensions.
    VedicExtensions([libc::c_uchar; 3]),
    /// U+{1D00..1D7F}: Phonetic Extensions.
    PhoneticExtensions([libc::c_uchar; 3]),
    /// U+{1D80..1DBF}: Phonetic Extensions Supplement.
    PhoneticExtensionsSupplement([libc::c_uchar; 3]),
    /// U+{1DC0..1DFF}: Combining Diacritical Marks Supplement.
    CombiningDiacriticalMarksSupplement([libc::c_uchar; 3]),
    /// U+{1E00..1EFF}: Latin Extended Additional.
    LatinExtendedAdditional([libc::c_uchar; 3]),
    /// U+{1F00..1FFF}: Greek Extended.
    GreekExtended([libc::c_uchar; 3]),
    /// U+{2000..206F}: General Punctuation.
    GeneralPunctuation([libc::c_uchar; 3]),
    /// U+{2070..209F}: Superscripts and Subscripts.
    SuperscriptsandSubscripts([libc::c_uchar; 3]),
    /// U+{20A0..20CF}: Currency Symbols.
    CurrencySymbols([libc::c_uchar; 3]),
    /// U+{20D0..20FF}: Combining Diacritical Marks for Symbols.
    CombiningDiacriticalMarksforSymbols([libc::c_uchar; 3]),
    /// U+{2100..214F}: Letterlike Symbols.
    LetterlikeSymbols([libc::c_uchar; 3]),
    /// U+{2150..218F}: Number Forms.
    NumberForms([libc::c_uchar; 3]),
    /// U+{2190..21FF}: Arrows.
    Arrows([libc::c_uchar; 3]),
    /// U+{2200..22FF}: Mathematical Operators.
    MathematicalOperators([libc::c_uchar; 3]),
    /// U+{2300..23FF}: Miscellaneous Technical.
    MiscellaneousTechnical([libc::c_uchar; 3]),
    /// U+{2400..243F}: Control Pictures.
    ControlPictures([libc::c_uchar; 3]),
    /// U+{2440..245F}: Optical Character Recognition.
    OpticalCharacterRecognition([libc::c_uchar; 3]),
    /// U+{2460..24FF}: Enclosed Alphanumerics.
    EnclosedAlphanumerics([libc::c_uchar; 3]),
    /// U+{2500..257F}: Box Drawing.
    BoxDrawing([libc::c_uchar; 3]),
    /// U+{2580..259F}: Block Elements.
    BlockElements([libc::c_uchar; 3]),
    /// U+{25A0..25FF}: Geometric Shapes.
    GeometricShapes([libc::c_uchar; 3]),
    /// U+{2600..26FF}: Miscellaneous Symbols.
    MiscellaneousSymbols([libc::c_uchar; 3]),
    /// U+{2700..27BF}: Dingbats.
    Dingbats([libc::c_uchar; 3]),
    /// U+{27C0..27EF}: Miscellaneous Mathematical Symbols-A.
    MiscellaneousMathematicalSymbolsA([libc::c_uchar; 3]),
    /// U+{27F0..27FF}: Supplemental Arrows-A.
    SupplementalArrowsA([libc::c_uchar; 3]),
    /// U+{2800..28FF}: Braille Patterns.
    BraillePatterns([libc::c_uchar; 3]),
    /// U+{2900..297F}: Supplemental Arrows-B.
    SupplementalArrowsB([libc::c_uchar; 3]),
    /// U+{2980..29FF}: Miscellaneous Mathematical Symbols-B.
    MiscellaneousMathematicalSymbolsB([libc::c_uchar; 3]),
    /// U+{2A00..2AFF}: Supplemental Mathematical Operators.
    SupplementalMathematicalOperators([libc::c_uchar; 3]),
    /// U+{2B00..2BFF}: Miscellaneous Symbols and Arrows.
    MiscellaneousSymbolsandArrows([libc::c_uchar; 3]),
    /// U+{2C00..2C5F}: Glagolitic.
    Glagolitic([libc::c_uchar; 3]),
    /// U+{2C60..2C7F}: Latin Extended-C.
    LatinExtendedC([libc::c_uchar; 3]),
    /// U+{2C80..2CFF}: Coptic.
    Coptic([libc::c_uchar; 3]),
    /// U+{2D00..2D2F}: Georgian Supplement.
    GeorgianSupplement([libc::c_uchar; 3]),
    /// U+{2D30..2D7F}: Tifinagh.
    Tifinagh([libc::c_uchar; 3]),
    /// U+{2D80..2DDF}: Ethiopic Extended.
    EthiopicExtended([libc::c_uchar; 3]),
    /// U+{2DE0..2DFF}: Cyrillic Extended-A.
    CyrillicExtendedA([libc::c_uchar; 3]),
    /// U+{2E00..2E7F}: Supplemental Punctuation.
    SupplementalPunctuation([libc::c_uchar; 3]),
    /// U+{2E80..2EFF}: CJK Radicals Supplement.
    CJKRadicalsSupplement([libc::c_uchar; 3]),
    /// U+{2F00..2FDF}: Kangxi Radicals.
    KangxiRadicals([libc::c_uchar; 3]),
    /// U+{2FF0..2FFF}: Ideographic Description Characters.
    IdeographicDescriptionCharacters([libc::c_uchar; 3]),
    /// U+{3000..303F}: CJK Symbols and Punctuation.
    CJKSymbolsandPunctuation([libc::c_uchar; 3]),
    /// U+{3040..309F}: Hiragana.
    Hiragana([libc::c_uchar; 3]),
    /// U+{30A0..30FF}: Katakana.
    Katakana([libc::c_uchar; 3]),
    /// U+{3100..312F}: Bopomofo.
    Bopomofo([libc::c_uchar; 3]),
    /// U+{3130..318F}: Hangul Compatibility Jamo.
    HangulCompatibilityJamo([libc::c_uchar; 3]),
    /// U+{3190..319F}: Kanbun.
    Kanbun([libc::c_uchar; 3]),
    /// U+{31A0..31BF}: Bopomofo Extended.
    BopomofoExtended([libc::c_uchar; 3]),
    /// U+{31C0..31EF}: CJK Strokes.
    CJKStrokes([libc::c_uchar; 3]),
    /// U+{31F0..31FF}: Katakana Phonetic Extensions.
    KatakanaPhoneticExtensions([libc::c_uchar; 3]),
    /// U+{3200..32FF}: Enclosed CJK Letters and Months.
    EnclosedCJKLettersandMonths([libc::c_uchar; 3]),
    /// U+{3300..33FF}: CJK Compatibility.
    CJKCompatibility([libc::c_uchar; 3]),
    /// U+{3400..4DBF}: CJK Unified Ideographs Extension A.
    CJKUnifiedIdeographsExtensionA([libc::c_uchar; 3]),
    /// U+{4DC0..4DFF}: Yijing Hexagram Symbols.
    YijingHexagramSymbols([libc::c_uchar; 3]),
    /// U+{4E00..9FFF}: CJK Unified Ideographs.
    CJKUnifiedIdeographs([libc::c_uchar; 3]),
    /// U+{A000..A48F}: Yi Syllables.
    YiSyllables([libc::c_uchar; 3]),
    /// U+{A490..A4CF}: Yi Radicals.
    YiRadicals([libc::c_uchar; 3]),
    /// U+{A4D0..A4FF}: Lisu.
    Lisu([libc::c_uchar; 3]),
    /// U+{A500..A63F}: Vai.
    Vai([libc::c_uchar; 3]),
    /// U+{A640..A69F}: Cyrillic Extended-B.
    CyrillicExtendedB([libc::c_uchar; 3]),
    /// U+{A6A0..A6FF}: Bamum.
    Bamum([libc::c_uchar; 3]),
    /// U+{A700..A71F}: Modifier Tone Letters.
    ModifierToneLetters([libc::c_uchar; 3]),
    /// U+{A720..A7FF}: Latin Extended-D.
    LatinExtendedD([libc::c_uchar; 3]),
    /// U+{A800..A82F}: Syloti Nagri.
    SylotiNagri([libc::c_uchar; 3]),
    /// U+{A830..A83F}: Common Indic Number Forms.
    CommonIndicNumberForms([libc::c_uchar; 3]),
    /// U+{A840..A87F}: Phags-pa.
    Phagspa([libc::c_uchar; 3]),
    /// U+{A880..A8DF}: Saurashtra.
    Saurashtra([libc::c_uchar; 3]),
    /// U+{A8E0..A8FF}: Devanagari Extended.
    DevanagariExtended([libc::c_uchar; 3]),
    /// U+{A900..A92F}: Kayah Li.
    KayahLi([libc::c_uchar; 3]),
    /// U+{A930..A95F}: Rejang.
    Rejang([libc::c_uchar; 3]),
    /// U+{A960..A97F}: Hangul Jamo Extended-A.
    HangulJamoExtendedA([libc::c_uchar; 3]),
    /// U+{A980..A9DF}: Javanese.
    Javanese([libc::c_uchar; 3]),
    /// U+{A9E0..A9FF}: Myanmar Extended-B.
    MyanmarExtendedB([libc::c_uchar; 3]),
    /// U+{AA00..AA5F}: Cham.
    Cham([libc::c_uchar; 3]),
    /// U+{AA60..AA7F}: Myanmar Extended-A.
    MyanmarExtendedA([libc::c_uchar; 3]),
    /// U+{AA80..AADF}: Tai Viet.
    TaiViet([libc::c_uchar; 3]),
    /// U+{AAE0..AAFF}: Meetei Mayek Extensions.
    MeeteiMayekExtensions([libc::c_uchar; 3]),
    /// U+{AB00..AB2F}: Ethiopic Extended-A.
    EthiopicExtendedA([libc::c_uchar; 3]),
    /// U+{AB30..AB6F}: Latin Extended-E.
    LatinExtendedE([libc::c_uchar; 3]),
    /// U+{AB70..ABBF}: Cherokee Supplement.
    CherokeeSupplement([libc::c_uchar; 3]),
    /// U+{ABC0..ABFF}: Meetei Mayek.
    MeeteiMayek([libc::c_uchar; 3]),
    /// U+{AC00..D7AF}: Hangul Syllables.
    HangulSyllables([libc::c_uchar; 3]),
    /// U+{D7B0..D7FF}: Hangul Jamo Extended-B.
    HangulJamoExtendedB([libc::c_uchar; 3]),
    /// U+{D800..DB7F}: High Surrogates.
    HighSurrogates([libc::c_uchar; 3]),
    /// U+{DB80..DBFF}: High Private Use Surrogates.
    HighPrivateUseSurrogates([libc::c_uchar; 3]),
    /// U+{DC00..DFFF}: Low Surrogates.
    LowSurrogates([libc::c_uchar; 3]),
    /// U+{E000..F8FF}: Private Use Area.
    PrivateUseArea([libc::c_uchar; 3]),
    /// U+{F900..FAFF}: CJK Compatibility Ideographs.
    CJKCompatibilityIdeographs([libc::c_uchar; 3]),
    /// U+{FB00..FB4F}: Alphabetic Presentation Forms.
    AlphabeticPresentationForms([libc::c_uchar; 3]),
    /// U+{FB50..FDFF}: Arabic Presentation Forms-A.
    ArabicPresentationFormsA([libc::c_uchar; 3]),
    /// U+{FE00..FE0F}: Variation Selectors.
    VariationSelectors([libc::c_uchar; 3]),
    /// U+{FE10..FE1F}: Vertical Forms.
    VerticalForms([libc::c_uchar; 3]),
    /// U+{FE20..FE2F}: Combining Half Marks.
    CombiningHalfMarks([libc::c_uchar; 3]),
    /// U+{FE30..FE4F}: CJK Compatibility Forms.
    CJKCompatibilityForms([libc::c_uchar; 3]),
    /// U+{FE50..FE6F}: Small Form Variants.
    SmallFormVariants([libc::c_uchar; 3]),
    /// U+{FE70..FEFF}: Arabic Presentation Forms-B.
    ArabicPresentationFormsB([libc::c_uchar; 3]),
    /// U+{FF00..FFEF}: Halfwidth and Fullwidth Forms.
    HalfwidthandFullwidthForms([libc::c_uchar; 3]),
    /// U+{FFF0..FFFF}: Specials.
    Specials([libc::c_uchar; 3]),
    /// U+{10000..1007F}: Linear B Syllabary.
    LinearBSyllabary([libc::c_uchar; 4]),
    /// U+{10080..100FF}: Linear B Ideograms.
    LinearBIdeograms([libc::c_uchar; 4]),
    /// U+{10100..1013F}: Aegean Numbers.
    AegeanNumbers([libc::c_uchar; 4]),
    /// U+{10140..1018F}: Ancient Greek Numbers.
    AncientGreekNumbers([libc::c_uchar; 4]),
    /// U+{10190..101CF}: Ancient Symbols.
    AncientSymbols([libc::c_uchar; 4]),
    /// U+{101D0..101FF}: Phaistos Disc.
    PhaistosDisc([libc::c_uchar; 4]),
    /// U+{10280..1029F}: Lycian.
    Lycian([libc::c_uchar; 4]),
    /// U+{102A0..102DF}: Carian.
    Carian([libc::c_uchar; 4]),
    /// U+{102E0..102FF}: Coptic Epact Numbers.
    CopticEpactNumbers([libc::c_uchar; 4]),
    /// U+{10300..1032F}: Old Italic.
    OldItalic([libc::c_uchar; 4]),
    /// U+{10330..1034F}: Gothic.
    Gothic([libc::c_uchar; 4]),
    /// U+{10350..1037F}: Old Permic.
    OldPermic([libc::c_uchar; 4]),
    /// U+{10380..1039F}: Ugaritic.
    Ugaritic([libc::c_uchar; 4]),
    /// U+{103A0..103DF}: Old Persian.
    OldPersian([libc::c_uchar; 4]),
    /// U+{10400..1044F}: Deseret.
    Deseret([libc::c_uchar; 4]),
    /// U+{10450..1047F}: Shavian.
    Shavian([libc::c_uchar; 4]),
    /// U+{10480..104AF}: Osmanya.
    Osmanya([libc::c_uchar; 4]),
    /// U+{10500..1052F}: Elbasan.
    Elbasan([libc::c_uchar; 4]),
    /// U+{10530..1056F}: Caucasian Albanian.
    CaucasianAlbanian([libc::c_uchar; 4]),
    /// U+{10600..1077F}: Linear A.
    LinearA([libc::c_uchar; 4]),
    /// U+{10800..1083F}: Cypriot Syllabary.
    CypriotSyllabary([libc::c_uchar; 4]),
    /// U+{10840..1085F}: Imperial Aramaic.
    ImperialAramaic([libc::c_uchar; 4]),
    /// U+{10860..1087F}: Palmyrene.
    Palmyrene([libc::c_uchar; 4]),
    /// U+{10880..108AF}: Nabataean.
    Nabataean([libc::c_uchar; 4]),
    /// U+{108E0..108FF}: Hatran.
    Hatran([libc::c_uchar; 4]),
    /// U+{10900..1091F}: Phoenician.
    Phoenician([libc::c_uchar; 4]),
    /// U+{10920..1093F}: Lydian.
    Lydian([libc::c_uchar; 4]),
    /// U+{10980..1099F}: Meroitic Hieroglyphs.
    MeroiticHieroglyphs([libc::c_uchar; 4]),
    /// U+{109A0..109FF}: Meroitic Cursive.
    MeroiticCursive([libc::c_uchar; 4]),
    /// U+{10A00..10A5F}: Kharoshthi.
    Kharoshthi([libc::c_uchar; 4]),
    /// U+{10A60..10A7F}: Old South Arabian.
    OldSouthArabian([libc::c_uchar; 4]),
    /// U+{10A80..10A9F}: Old North Arabian.
    OldNorthArabian([libc::c_uchar; 4]),
    /// U+{10AC0..10AFF}: Manichaean.
    Manichaean([libc::c_uchar; 4]),
    /// U+{10B00..10B3F}: Avestan.
    Avestan([libc::c_uchar; 4]),
    /// U+{10B40..10B5F}: Inscriptional Parthian.
    InscriptionalParthian([libc::c_uchar; 4]),
    /// U+{10B60..10B7F}: Inscriptional Pahlavi.
    InscriptionalPahlavi([libc::c_uchar; 4]),
    /// U+{10B80..10BAF}: Psalter Pahlavi.
    PsalterPahlavi([libc::c_uchar; 4]),
    /// U+{10C00..10C4F}: Old Turkic.
    OldTurkic([libc::c_uchar; 4]),
    /// U+{10C80..10CFF}: Old Hungarian.
    OldHungarian([libc::c_uchar; 4]),
    /// U+{10E60..10E7F}: Rumi Numeral Symbols.
    RumiNumeralSymbols([libc::c_uchar; 4]),
    /// U+{11000..1107F}: Brahmi.
    Brahmi([libc::c_uchar; 4]),
    /// U+{11080..110CF}: Kaithi.
    Kaithi([libc::c_uchar; 4]),
    /// U+{110D0..110FF}: Sora Sompeng.
    SoraSompeng([libc::c_uchar; 4]),
    /// U+{11100..1114F}: Chakma.
    Chakma([libc::c_uchar; 4]),
    /// U+{11150..1117F}: Mahajani.
    Mahajani([libc::c_uchar; 4]),
    /// U+{11180..111DF}: Sharada.
    Sharada([libc::c_uchar; 4]),
    /// U+{111E0..111FF}: Sinhala Archaic Numbers.
    SinhalaArchaicNumbers([libc::c_uchar; 4]),
    /// U+{11200..1124F}: Khojki.
    Khojki([libc::c_uchar; 4]),
    /// U+{11280..112AF}: Multani.
    Multani([libc::c_uchar; 4]),
    /// U+{112B0..112FF}: Khudawadi.
    Khudawadi([libc::c_uchar; 4]),
    /// U+{11300..1137F}: Grantha.
    Grantha([libc::c_uchar; 4]),
    /// U+{11480..114DF}: Tirhuta.
    Tirhuta([libc::c_uchar; 4]),
    /// U+{11580..115FF}: Siddham.
    Siddham([libc::c_uchar; 4]),
    /// U+{11600..1165F}: Modi.
    Modi([libc::c_uchar; 4]),
    /// U+{11680..116CF}: Takri.
    Takri([libc::c_uchar; 4]),
    /// U+{11700..1173F}: Ahom.
    Ahom([libc::c_uchar; 4]),
    /// U+{118A0..118FF}: Warang Citi.
    WarangCiti([libc::c_uchar; 4]),
    /// U+{11AC0..11AFF}: Pau Cin Hau.
    PauCinHau([libc::c_uchar; 4]),
    /// U+{12000..123FF}: Cuneiform.
    Cuneiform([libc::c_uchar; 4]),
    /// U+{12400..1247F}: Cuneiform Numbers and Punctuation.
    CuneiformNumbersandPunctuation([libc::c_uchar; 4]),
    /// U+{12480..1254F}: Early Dynastic Cuneiform.
    EarlyDynasticCuneiform([libc::c_uchar; 4]),
    /// U+{13000..1342F}: Egyptian Hieroglyphs.
    EgyptianHieroglyphs([libc::c_uchar; 4]),
    /// U+{14400..1467F}: Anatolian Hieroglyphs.
    AnatolianHieroglyphs([libc::c_uchar; 4]),
    /// U+{16800..16A3F}: Bamum Supplement.
    BamumSupplement([libc::c_uchar; 4]),
    /// U+{16A40..16A6F}: Mro.
    Mro([libc::c_uchar; 4]),
    /// U+{16AD0..16AFF}: Bassa Vah.
    BassaVah([libc::c_uchar; 4]),
    /// U+{16B00..16B8F}: Pahawh Hmong.
    PahawhHmong([libc::c_uchar; 4]),
    /// U+{16F00..16F9F}: Miao.
    Miao([libc::c_uchar; 4]),
    /// U+{1B000..1B0FF}: Kana Supplement.
    KanaSupplement([libc::c_uchar; 4]),
    /// U+{1BC00..1BC9F}: Duployan.
    Duployan([libc::c_uchar; 4]),
    /// U+{1BCA0..1BCAF}: Shorthand Format Controls.
    ShorthandFormatControls([libc::c_uchar; 4]),
    /// U+{1D000..1D0FF}: Byzantine Musical Symbols.
    ByzantineMusicalSymbols([libc::c_uchar; 4]),
    /// U+{1D100..1D1FF}: Musical Symbols.
    MusicalSymbols([libc::c_uchar; 4]),
    /// U+{1D200..1D24F}: Ancient Greek Musical Notation.
    AncientGreekMusicalNotation([libc::c_uchar; 4]),
    /// U+{1D300..1D35F}: Tai Xuan Jing Symbols.
    TaiXuanJingSymbols([libc::c_uchar; 4]),
    /// U+{1D360..1D37F}: Counting Rod Numerals.
    CountingRodNumerals([libc::c_uchar; 4]),
    /// U+{1D400..1D7FF}: Mathematical Alphanumeric Symbols.
    MathematicalAlphanumericSymbols([libc::c_uchar; 4]),
    /// U+{1D800..1DAAF}: Sutton SignWriting.
    SuttonSignWriting([libc::c_uchar; 4]),
    /// U+{1E800..1E8DF}: Mende Kikakui.
    MendeKikakui([libc::c_uchar; 4]),
    /// U+{1EE00..1EEFF}: Arabic Mathematical Alphabetic Symbols.
    ArabicMathematicalAlphabeticSymbols([libc::c_uchar; 4]),
    /// U+{1F000..1F02F}: Mahjong Tiles.
    MahjongTiles([libc::c_uchar; 4]),
    /// U+{1F030..1F09F}: Domino Tiles.
    DominoTiles([libc::c_uchar; 4]),
    /// U+{1F0A0..1F0FF}: Playing Cards.
    PlayingCards([libc::c_uchar; 4]),
    /// U+{1F100..1F1FF}: Enclosed Alphanumeric Supplement.
    EnclosedAlphanumericSupplement([libc::c_uchar; 4]),
    /// U+{1F200..1F2FF}: Enclosed Ideographic Supplement.
    EnclosedIdeographicSupplement([libc::c_uchar; 4]),
    /// U+{1F300..1F5FF}: Miscellaneous Symbols and Pictographs.
    MiscellaneousSymbolsandPictographs([libc::c_uchar; 4]),
    /// U+{1F600..1F64F}: Emoticons.
    Emoticons([libc::c_uchar; 4]),
    /// U+{1F650..1F67F}: Ornamental Dingbats.
    OrnamentalDingbats([libc::c_uchar; 4]),
    /// U+{1F680..1F6FF}: Transport and Map Symbols.
    TransportandMapSymbols([libc::c_uchar; 4]),
    /// U+{1F700..1F77F}: Alchemical Symbols.
    AlchemicalSymbols([libc::c_uchar; 4]),
    /// U+{1F780..1F7FF}: Geometric Shapes Extended.
    GeometricShapesExtended([libc::c_uchar; 4]),
    /// U+{1F800..1F8FF}: Supplemental Arrows-C.
    SupplementalArrowsC([libc::c_uchar; 4]),
    /// U+{1F900..1F9FF}: Supplemental Symbols and Pictographs.
    SupplementalSymbolsandPictographs([libc::c_uchar; 4]),
    /// U+{20000..2A6DF}: CJK Unified Ideographs Extension B.
    CjkUnifiedIdeographsExtensionB([libc::c_uchar; 4]),
    /// U+{2A700..2B73F}: CJK Unified Ideographs Extension C.
    CjkUnifiedIdeographsExtensionC([libc::c_uchar; 4]),
    /// U+{2B740..2B81F}: CJK Unified Ideographs Extension D.
    CjkUnifiedIdeographsExtensionD([libc::c_uchar; 4]),
    /// U+{2B820..2CEAF}: CJK Unified Ideographs Extension E.
    CjkUnifiedIdeographsExtensionE([libc::c_uchar; 4]),
    /// U+{2F800..2FA1F}: CJK Compatibility Ideographs Supplement.
    CjkCompatibilityIdeographsSupplement([libc::c_uchar; 4]),
    /// U+{E0000..E007F}: Tags.
    Tags([libc::c_uchar; 4]),
    /// U+{E0100..E01EF}: Variation Selectors Supplement.
    VariationSelectorsSupplement([libc::c_uchar; 4]),
    /// U+{F0000..FFFFF}: Supplementary Private Use Area-A.
    SupplementaryPrivateUseAreaA([libc::c_uchar; 4]),
    /// U+{100000..10FFFF}: Supplementary Private Use Area-B.
    SupplementaryPrivateUseAreaB([libc::c_uchar; 4]),
}
