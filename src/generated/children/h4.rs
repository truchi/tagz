// 🤖 This file is generated!

use crate::*;
/// The `<h4>` element's children.
#[derive(Clone)]
pub enum Heading4Child {
    Abbreviation(Abbreviation),
    Anchor(Anchor),
    Audio(Audio),
    BidirectionalIsolate(BidirectionalIsolate),
    BidirectionalOverride(BidirectionalOverride),
    Bold(Bold),
    Button(Button),
    Canvas(Canvas),
    Cite(Cite),
    Code(Code),
    Custom(Custom),
    Data(Data),
    DataList(DataList),
    Definition(Definition),
    Deleted(Deleted),
    Embed(Embed),
    Emphasis(Emphasis),
    Image(Image),
    InlineFrame(InlineFrame),
    Input(Input),
    Inserted(Inserted),
    Italic(Italic),
    Keyboard(Keyboard),
    Label(Label),
    LineBreak(LineBreak),
    Link(Link),
    Map(Map),
    MapArea(MapArea),
    Mark(Mark),
    Metadata(Metadata),
    Meter(Meter),
    NoScript(NoScript),
    Object(Object),
    Output(Output),
    Picture(Picture),
    Progress(Progress),
    Quote(Quote),
    Ruby(Ruby),
    Sample(Sample),
    Script(Script),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Span(Span),
    StrikeThrough(StrikeThrough),
    Strong(Strong),
    SubScript(SubScript),
    SupScript(SupScript),
    Template(Template),
    TextArea(TextArea),
    Time(Time),
    Underline(Underline),
    Variable(Variable),
    Video(Video),
    WordBreak(WordBreak),
    Text(CowStr),
}
impl From<Abbreviation> for Heading4Child {
    fn from(child: Abbreviation) -> Self {
        Heading4Child::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for Heading4Child {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        Heading4Child::Abbreviation(builder.build())
    }
}
impl From<Anchor> for Heading4Child {
    fn from(child: Anchor) -> Self {
        Heading4Child::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for Heading4Child {
    fn from(builder: builders::AnchorBuilder) -> Self {
        Heading4Child::Anchor(builder.build())
    }
}
impl From<Audio> for Heading4Child {
    fn from(child: Audio) -> Self {
        Heading4Child::Audio(child)
    }
}
impl From<builders::AudioBuilder> for Heading4Child {
    fn from(builder: builders::AudioBuilder) -> Self {
        Heading4Child::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for Heading4Child {
    fn from(child: BidirectionalIsolate) -> Self {
        Heading4Child::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for Heading4Child {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        Heading4Child::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for Heading4Child {
    fn from(child: BidirectionalOverride) -> Self {
        Heading4Child::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for Heading4Child {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        Heading4Child::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for Heading4Child {
    fn from(child: Bold) -> Self {
        Heading4Child::Bold(child)
    }
}
impl From<builders::BoldBuilder> for Heading4Child {
    fn from(builder: builders::BoldBuilder) -> Self {
        Heading4Child::Bold(builder.build())
    }
}
impl From<Button> for Heading4Child {
    fn from(child: Button) -> Self {
        Heading4Child::Button(child)
    }
}
impl From<builders::ButtonBuilder> for Heading4Child {
    fn from(builder: builders::ButtonBuilder) -> Self {
        Heading4Child::Button(builder.build())
    }
}
impl From<Canvas> for Heading4Child {
    fn from(child: Canvas) -> Self {
        Heading4Child::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for Heading4Child {
    fn from(builder: builders::CanvasBuilder) -> Self {
        Heading4Child::Canvas(builder.build())
    }
}
impl From<Cite> for Heading4Child {
    fn from(child: Cite) -> Self {
        Heading4Child::Cite(child)
    }
}
impl From<builders::CiteBuilder> for Heading4Child {
    fn from(builder: builders::CiteBuilder) -> Self {
        Heading4Child::Cite(builder.build())
    }
}
impl From<Code> for Heading4Child {
    fn from(child: Code) -> Self {
        Heading4Child::Code(child)
    }
}
impl From<builders::CodeBuilder> for Heading4Child {
    fn from(builder: builders::CodeBuilder) -> Self {
        Heading4Child::Code(builder.build())
    }
}
impl From<Custom> for Heading4Child {
    fn from(child: Custom) -> Self {
        Heading4Child::Custom(child)
    }
}
impl From<builders::CustomBuilder> for Heading4Child {
    fn from(builder: builders::CustomBuilder) -> Self {
        Heading4Child::Custom(builder.build())
    }
}
impl From<Data> for Heading4Child {
    fn from(child: Data) -> Self {
        Heading4Child::Data(child)
    }
}
impl From<builders::DataBuilder> for Heading4Child {
    fn from(builder: builders::DataBuilder) -> Self {
        Heading4Child::Data(builder.build())
    }
}
impl From<DataList> for Heading4Child {
    fn from(child: DataList) -> Self {
        Heading4Child::DataList(child)
    }
}
impl From<builders::DataListBuilder> for Heading4Child {
    fn from(builder: builders::DataListBuilder) -> Self {
        Heading4Child::DataList(builder.build())
    }
}
impl From<Definition> for Heading4Child {
    fn from(child: Definition) -> Self {
        Heading4Child::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for Heading4Child {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        Heading4Child::Definition(builder.build())
    }
}
impl From<Deleted> for Heading4Child {
    fn from(child: Deleted) -> Self {
        Heading4Child::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for Heading4Child {
    fn from(builder: builders::DeletedBuilder) -> Self {
        Heading4Child::Deleted(builder.build())
    }
}
impl From<Embed> for Heading4Child {
    fn from(child: Embed) -> Self {
        Heading4Child::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for Heading4Child {
    fn from(builder: builders::EmbedBuilder) -> Self {
        Heading4Child::Embed(builder.build())
    }
}
impl From<Emphasis> for Heading4Child {
    fn from(child: Emphasis) -> Self {
        Heading4Child::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for Heading4Child {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        Heading4Child::Emphasis(builder.build())
    }
}
impl From<Image> for Heading4Child {
    fn from(child: Image) -> Self {
        Heading4Child::Image(child)
    }
}
impl From<builders::ImageBuilder> for Heading4Child {
    fn from(builder: builders::ImageBuilder) -> Self {
        Heading4Child::Image(builder.build())
    }
}
impl From<InlineFrame> for Heading4Child {
    fn from(child: InlineFrame) -> Self {
        Heading4Child::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for Heading4Child {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        Heading4Child::InlineFrame(builder.build())
    }
}
impl From<Input> for Heading4Child {
    fn from(child: Input) -> Self {
        Heading4Child::Input(child)
    }
}
impl From<builders::InputBuilder> for Heading4Child {
    fn from(builder: builders::InputBuilder) -> Self {
        Heading4Child::Input(builder.build())
    }
}
impl From<Inserted> for Heading4Child {
    fn from(child: Inserted) -> Self {
        Heading4Child::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for Heading4Child {
    fn from(builder: builders::InsertedBuilder) -> Self {
        Heading4Child::Inserted(builder.build())
    }
}
impl From<Italic> for Heading4Child {
    fn from(child: Italic) -> Self {
        Heading4Child::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for Heading4Child {
    fn from(builder: builders::ItalicBuilder) -> Self {
        Heading4Child::Italic(builder.build())
    }
}
impl From<Keyboard> for Heading4Child {
    fn from(child: Keyboard) -> Self {
        Heading4Child::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for Heading4Child {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        Heading4Child::Keyboard(builder.build())
    }
}
impl From<Label> for Heading4Child {
    fn from(child: Label) -> Self {
        Heading4Child::Label(child)
    }
}
impl From<builders::LabelBuilder> for Heading4Child {
    fn from(builder: builders::LabelBuilder) -> Self {
        Heading4Child::Label(builder.build())
    }
}
impl From<LineBreak> for Heading4Child {
    fn from(child: LineBreak) -> Self {
        Heading4Child::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for Heading4Child {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        Heading4Child::LineBreak(builder.build())
    }
}
impl From<Link> for Heading4Child {
    fn from(child: Link) -> Self {
        Heading4Child::Link(child)
    }
}
impl From<builders::LinkBuilder> for Heading4Child {
    fn from(builder: builders::LinkBuilder) -> Self {
        Heading4Child::Link(builder.build())
    }
}
impl From<Map> for Heading4Child {
    fn from(child: Map) -> Self {
        Heading4Child::Map(child)
    }
}
impl From<builders::MapBuilder> for Heading4Child {
    fn from(builder: builders::MapBuilder) -> Self {
        Heading4Child::Map(builder.build())
    }
}
impl From<MapArea> for Heading4Child {
    fn from(child: MapArea) -> Self {
        Heading4Child::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for Heading4Child {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        Heading4Child::MapArea(builder.build())
    }
}
impl From<Mark> for Heading4Child {
    fn from(child: Mark) -> Self {
        Heading4Child::Mark(child)
    }
}
impl From<builders::MarkBuilder> for Heading4Child {
    fn from(builder: builders::MarkBuilder) -> Self {
        Heading4Child::Mark(builder.build())
    }
}
impl From<Metadata> for Heading4Child {
    fn from(child: Metadata) -> Self {
        Heading4Child::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for Heading4Child {
    fn from(builder: builders::MetadataBuilder) -> Self {
        Heading4Child::Metadata(builder.build())
    }
}
impl From<Meter> for Heading4Child {
    fn from(child: Meter) -> Self {
        Heading4Child::Meter(child)
    }
}
impl From<builders::MeterBuilder> for Heading4Child {
    fn from(builder: builders::MeterBuilder) -> Self {
        Heading4Child::Meter(builder.build())
    }
}
impl From<NoScript> for Heading4Child {
    fn from(child: NoScript) -> Self {
        Heading4Child::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for Heading4Child {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        Heading4Child::NoScript(builder.build())
    }
}
impl From<Object> for Heading4Child {
    fn from(child: Object) -> Self {
        Heading4Child::Object(child)
    }
}
impl From<builders::ObjectBuilder> for Heading4Child {
    fn from(builder: builders::ObjectBuilder) -> Self {
        Heading4Child::Object(builder.build())
    }
}
impl From<Output> for Heading4Child {
    fn from(child: Output) -> Self {
        Heading4Child::Output(child)
    }
}
impl From<builders::OutputBuilder> for Heading4Child {
    fn from(builder: builders::OutputBuilder) -> Self {
        Heading4Child::Output(builder.build())
    }
}
impl From<Picture> for Heading4Child {
    fn from(child: Picture) -> Self {
        Heading4Child::Picture(child)
    }
}
impl From<builders::PictureBuilder> for Heading4Child {
    fn from(builder: builders::PictureBuilder) -> Self {
        Heading4Child::Picture(builder.build())
    }
}
impl From<Progress> for Heading4Child {
    fn from(child: Progress) -> Self {
        Heading4Child::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for Heading4Child {
    fn from(builder: builders::ProgressBuilder) -> Self {
        Heading4Child::Progress(builder.build())
    }
}
impl From<Quote> for Heading4Child {
    fn from(child: Quote) -> Self {
        Heading4Child::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for Heading4Child {
    fn from(builder: builders::QuoteBuilder) -> Self {
        Heading4Child::Quote(builder.build())
    }
}
impl From<Ruby> for Heading4Child {
    fn from(child: Ruby) -> Self {
        Heading4Child::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for Heading4Child {
    fn from(builder: builders::RubyBuilder) -> Self {
        Heading4Child::Ruby(builder.build())
    }
}
impl From<Sample> for Heading4Child {
    fn from(child: Sample) -> Self {
        Heading4Child::Sample(child)
    }
}
impl From<builders::SampleBuilder> for Heading4Child {
    fn from(builder: builders::SampleBuilder) -> Self {
        Heading4Child::Sample(builder.build())
    }
}
impl From<Script> for Heading4Child {
    fn from(child: Script) -> Self {
        Heading4Child::Script(child)
    }
}
impl From<builders::ScriptBuilder> for Heading4Child {
    fn from(builder: builders::ScriptBuilder) -> Self {
        Heading4Child::Script(builder.build())
    }
}
impl From<Select> for Heading4Child {
    fn from(child: Select) -> Self {
        Heading4Child::Select(child)
    }
}
impl From<builders::SelectBuilder> for Heading4Child {
    fn from(builder: builders::SelectBuilder) -> Self {
        Heading4Child::Select(builder.build())
    }
}
impl From<Slot> for Heading4Child {
    fn from(child: Slot) -> Self {
        Heading4Child::Slot(child)
    }
}
impl From<builders::SlotBuilder> for Heading4Child {
    fn from(builder: builders::SlotBuilder) -> Self {
        Heading4Child::Slot(builder.build())
    }
}
impl From<Small> for Heading4Child {
    fn from(child: Small) -> Self {
        Heading4Child::Small(child)
    }
}
impl From<builders::SmallBuilder> for Heading4Child {
    fn from(builder: builders::SmallBuilder) -> Self {
        Heading4Child::Small(builder.build())
    }
}
impl From<Span> for Heading4Child {
    fn from(child: Span) -> Self {
        Heading4Child::Span(child)
    }
}
impl From<builders::SpanBuilder> for Heading4Child {
    fn from(builder: builders::SpanBuilder) -> Self {
        Heading4Child::Span(builder.build())
    }
}
impl From<StrikeThrough> for Heading4Child {
    fn from(child: StrikeThrough) -> Self {
        Heading4Child::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for Heading4Child {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        Heading4Child::StrikeThrough(builder.build())
    }
}
impl From<Strong> for Heading4Child {
    fn from(child: Strong) -> Self {
        Heading4Child::Strong(child)
    }
}
impl From<builders::StrongBuilder> for Heading4Child {
    fn from(builder: builders::StrongBuilder) -> Self {
        Heading4Child::Strong(builder.build())
    }
}
impl From<SubScript> for Heading4Child {
    fn from(child: SubScript) -> Self {
        Heading4Child::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for Heading4Child {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        Heading4Child::SubScript(builder.build())
    }
}
impl From<SupScript> for Heading4Child {
    fn from(child: SupScript) -> Self {
        Heading4Child::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for Heading4Child {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        Heading4Child::SupScript(builder.build())
    }
}
impl From<Template> for Heading4Child {
    fn from(child: Template) -> Self {
        Heading4Child::Template(child)
    }
}
impl From<builders::TemplateBuilder> for Heading4Child {
    fn from(builder: builders::TemplateBuilder) -> Self {
        Heading4Child::Template(builder.build())
    }
}
impl From<TextArea> for Heading4Child {
    fn from(child: TextArea) -> Self {
        Heading4Child::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for Heading4Child {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        Heading4Child::TextArea(builder.build())
    }
}
impl From<Time> for Heading4Child {
    fn from(child: Time) -> Self {
        Heading4Child::Time(child)
    }
}
impl From<builders::TimeBuilder> for Heading4Child {
    fn from(builder: builders::TimeBuilder) -> Self {
        Heading4Child::Time(builder.build())
    }
}
impl From<Underline> for Heading4Child {
    fn from(child: Underline) -> Self {
        Heading4Child::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for Heading4Child {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        Heading4Child::Underline(builder.build())
    }
}
impl From<Variable> for Heading4Child {
    fn from(child: Variable) -> Self {
        Heading4Child::Variable(child)
    }
}
impl From<builders::VariableBuilder> for Heading4Child {
    fn from(builder: builders::VariableBuilder) -> Self {
        Heading4Child::Variable(builder.build())
    }
}
impl From<Video> for Heading4Child {
    fn from(child: Video) -> Self {
        Heading4Child::Video(child)
    }
}
impl From<builders::VideoBuilder> for Heading4Child {
    fn from(builder: builders::VideoBuilder) -> Self {
        Heading4Child::Video(builder.build())
    }
}
impl From<WordBreak> for Heading4Child {
    fn from(child: WordBreak) -> Self {
        Heading4Child::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for Heading4Child {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        Heading4Child::WordBreak(builder.build())
    }
}
impl From<&'static str> for Heading4Child {
    fn from(s: &'static str) -> Self {
        Heading4Child::Text(s.into())
    }
}
impl From<String> for Heading4Child {
    fn from(s: String) -> Self {
        Heading4Child::Text(s.into())
    }
}
impl From<CowStr> for Heading4Child {
    fn from(s: CowStr) -> Self {
        Heading4Child::Text(s)
    }
}
impl std::fmt::Debug for Heading4Child {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heading4Child::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Anchor(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Audio(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Bold(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Button(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Canvas(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Cite(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Code(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Custom(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Data(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::DataList(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Definition(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Deleted(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Embed(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Image(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Input(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Inserted(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Italic(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Label(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Link(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Map(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::MapArea(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Mark(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Metadata(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Meter(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::NoScript(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Object(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Output(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Picture(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Progress(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Quote(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Ruby(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Sample(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Script(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Select(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Slot(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Small(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Span(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Strong(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::SubScript(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::SupScript(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Template(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::TextArea(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Time(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Underline(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Variable(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Video(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            Heading4Child::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for Heading4Child {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heading4Child::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Anchor(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Audio(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            Heading4Child::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            Heading4Child::Bold(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Button(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Canvas(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Cite(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Code(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Custom(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Data(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::DataList(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Definition(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Deleted(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Embed(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Emphasis(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Image(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Input(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Inserted(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Italic(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Keyboard(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Label(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::LineBreak(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Link(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Map(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::MapArea(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Mark(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Metadata(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Meter(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::NoScript(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Object(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Output(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Picture(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Progress(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Quote(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Ruby(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Sample(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Script(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Select(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Slot(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Small(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Span(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Strong(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::SubScript(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::SupScript(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Template(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::TextArea(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Time(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Underline(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Variable(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Video(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::WordBreak(child) => std::fmt::Display::fmt(child, f),
            Heading4Child::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
