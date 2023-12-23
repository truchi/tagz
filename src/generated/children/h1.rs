// 🤖 This file is generated!

use crate::*;
/// The `<h1>` element's children.
#[derive(Clone)]
pub enum Heading1Child {
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
impl From<Abbreviation> for Heading1Child {
    fn from(child: Abbreviation) -> Self {
        Heading1Child::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for Heading1Child {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        Heading1Child::Abbreviation(builder.build())
    }
}
impl From<Anchor> for Heading1Child {
    fn from(child: Anchor) -> Self {
        Heading1Child::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for Heading1Child {
    fn from(builder: builders::AnchorBuilder) -> Self {
        Heading1Child::Anchor(builder.build())
    }
}
impl From<Audio> for Heading1Child {
    fn from(child: Audio) -> Self {
        Heading1Child::Audio(child)
    }
}
impl From<builders::AudioBuilder> for Heading1Child {
    fn from(builder: builders::AudioBuilder) -> Self {
        Heading1Child::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for Heading1Child {
    fn from(child: BidirectionalIsolate) -> Self {
        Heading1Child::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for Heading1Child {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        Heading1Child::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for Heading1Child {
    fn from(child: BidirectionalOverride) -> Self {
        Heading1Child::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for Heading1Child {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        Heading1Child::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for Heading1Child {
    fn from(child: Bold) -> Self {
        Heading1Child::Bold(child)
    }
}
impl From<builders::BoldBuilder> for Heading1Child {
    fn from(builder: builders::BoldBuilder) -> Self {
        Heading1Child::Bold(builder.build())
    }
}
impl From<Button> for Heading1Child {
    fn from(child: Button) -> Self {
        Heading1Child::Button(child)
    }
}
impl From<builders::ButtonBuilder> for Heading1Child {
    fn from(builder: builders::ButtonBuilder) -> Self {
        Heading1Child::Button(builder.build())
    }
}
impl From<Canvas> for Heading1Child {
    fn from(child: Canvas) -> Self {
        Heading1Child::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for Heading1Child {
    fn from(builder: builders::CanvasBuilder) -> Self {
        Heading1Child::Canvas(builder.build())
    }
}
impl From<Cite> for Heading1Child {
    fn from(child: Cite) -> Self {
        Heading1Child::Cite(child)
    }
}
impl From<builders::CiteBuilder> for Heading1Child {
    fn from(builder: builders::CiteBuilder) -> Self {
        Heading1Child::Cite(builder.build())
    }
}
impl From<Code> for Heading1Child {
    fn from(child: Code) -> Self {
        Heading1Child::Code(child)
    }
}
impl From<builders::CodeBuilder> for Heading1Child {
    fn from(builder: builders::CodeBuilder) -> Self {
        Heading1Child::Code(builder.build())
    }
}
impl From<Custom> for Heading1Child {
    fn from(child: Custom) -> Self {
        Heading1Child::Custom(child)
    }
}
impl From<builders::CustomBuilder> for Heading1Child {
    fn from(builder: builders::CustomBuilder) -> Self {
        Heading1Child::Custom(builder.build())
    }
}
impl From<Data> for Heading1Child {
    fn from(child: Data) -> Self {
        Heading1Child::Data(child)
    }
}
impl From<builders::DataBuilder> for Heading1Child {
    fn from(builder: builders::DataBuilder) -> Self {
        Heading1Child::Data(builder.build())
    }
}
impl From<DataList> for Heading1Child {
    fn from(child: DataList) -> Self {
        Heading1Child::DataList(child)
    }
}
impl From<builders::DataListBuilder> for Heading1Child {
    fn from(builder: builders::DataListBuilder) -> Self {
        Heading1Child::DataList(builder.build())
    }
}
impl From<Definition> for Heading1Child {
    fn from(child: Definition) -> Self {
        Heading1Child::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for Heading1Child {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        Heading1Child::Definition(builder.build())
    }
}
impl From<Deleted> for Heading1Child {
    fn from(child: Deleted) -> Self {
        Heading1Child::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for Heading1Child {
    fn from(builder: builders::DeletedBuilder) -> Self {
        Heading1Child::Deleted(builder.build())
    }
}
impl From<Embed> for Heading1Child {
    fn from(child: Embed) -> Self {
        Heading1Child::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for Heading1Child {
    fn from(builder: builders::EmbedBuilder) -> Self {
        Heading1Child::Embed(builder.build())
    }
}
impl From<Emphasis> for Heading1Child {
    fn from(child: Emphasis) -> Self {
        Heading1Child::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for Heading1Child {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        Heading1Child::Emphasis(builder.build())
    }
}
impl From<Image> for Heading1Child {
    fn from(child: Image) -> Self {
        Heading1Child::Image(child)
    }
}
impl From<builders::ImageBuilder> for Heading1Child {
    fn from(builder: builders::ImageBuilder) -> Self {
        Heading1Child::Image(builder.build())
    }
}
impl From<InlineFrame> for Heading1Child {
    fn from(child: InlineFrame) -> Self {
        Heading1Child::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for Heading1Child {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        Heading1Child::InlineFrame(builder.build())
    }
}
impl From<Input> for Heading1Child {
    fn from(child: Input) -> Self {
        Heading1Child::Input(child)
    }
}
impl From<builders::InputBuilder> for Heading1Child {
    fn from(builder: builders::InputBuilder) -> Self {
        Heading1Child::Input(builder.build())
    }
}
impl From<Inserted> for Heading1Child {
    fn from(child: Inserted) -> Self {
        Heading1Child::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for Heading1Child {
    fn from(builder: builders::InsertedBuilder) -> Self {
        Heading1Child::Inserted(builder.build())
    }
}
impl From<Italic> for Heading1Child {
    fn from(child: Italic) -> Self {
        Heading1Child::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for Heading1Child {
    fn from(builder: builders::ItalicBuilder) -> Self {
        Heading1Child::Italic(builder.build())
    }
}
impl From<Keyboard> for Heading1Child {
    fn from(child: Keyboard) -> Self {
        Heading1Child::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for Heading1Child {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        Heading1Child::Keyboard(builder.build())
    }
}
impl From<Label> for Heading1Child {
    fn from(child: Label) -> Self {
        Heading1Child::Label(child)
    }
}
impl From<builders::LabelBuilder> for Heading1Child {
    fn from(builder: builders::LabelBuilder) -> Self {
        Heading1Child::Label(builder.build())
    }
}
impl From<LineBreak> for Heading1Child {
    fn from(child: LineBreak) -> Self {
        Heading1Child::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for Heading1Child {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        Heading1Child::LineBreak(builder.build())
    }
}
impl From<Link> for Heading1Child {
    fn from(child: Link) -> Self {
        Heading1Child::Link(child)
    }
}
impl From<builders::LinkBuilder> for Heading1Child {
    fn from(builder: builders::LinkBuilder) -> Self {
        Heading1Child::Link(builder.build())
    }
}
impl From<Map> for Heading1Child {
    fn from(child: Map) -> Self {
        Heading1Child::Map(child)
    }
}
impl From<builders::MapBuilder> for Heading1Child {
    fn from(builder: builders::MapBuilder) -> Self {
        Heading1Child::Map(builder.build())
    }
}
impl From<MapArea> for Heading1Child {
    fn from(child: MapArea) -> Self {
        Heading1Child::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for Heading1Child {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        Heading1Child::MapArea(builder.build())
    }
}
impl From<Mark> for Heading1Child {
    fn from(child: Mark) -> Self {
        Heading1Child::Mark(child)
    }
}
impl From<builders::MarkBuilder> for Heading1Child {
    fn from(builder: builders::MarkBuilder) -> Self {
        Heading1Child::Mark(builder.build())
    }
}
impl From<Metadata> for Heading1Child {
    fn from(child: Metadata) -> Self {
        Heading1Child::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for Heading1Child {
    fn from(builder: builders::MetadataBuilder) -> Self {
        Heading1Child::Metadata(builder.build())
    }
}
impl From<Meter> for Heading1Child {
    fn from(child: Meter) -> Self {
        Heading1Child::Meter(child)
    }
}
impl From<builders::MeterBuilder> for Heading1Child {
    fn from(builder: builders::MeterBuilder) -> Self {
        Heading1Child::Meter(builder.build())
    }
}
impl From<NoScript> for Heading1Child {
    fn from(child: NoScript) -> Self {
        Heading1Child::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for Heading1Child {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        Heading1Child::NoScript(builder.build())
    }
}
impl From<Object> for Heading1Child {
    fn from(child: Object) -> Self {
        Heading1Child::Object(child)
    }
}
impl From<builders::ObjectBuilder> for Heading1Child {
    fn from(builder: builders::ObjectBuilder) -> Self {
        Heading1Child::Object(builder.build())
    }
}
impl From<Output> for Heading1Child {
    fn from(child: Output) -> Self {
        Heading1Child::Output(child)
    }
}
impl From<builders::OutputBuilder> for Heading1Child {
    fn from(builder: builders::OutputBuilder) -> Self {
        Heading1Child::Output(builder.build())
    }
}
impl From<Picture> for Heading1Child {
    fn from(child: Picture) -> Self {
        Heading1Child::Picture(child)
    }
}
impl From<builders::PictureBuilder> for Heading1Child {
    fn from(builder: builders::PictureBuilder) -> Self {
        Heading1Child::Picture(builder.build())
    }
}
impl From<Progress> for Heading1Child {
    fn from(child: Progress) -> Self {
        Heading1Child::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for Heading1Child {
    fn from(builder: builders::ProgressBuilder) -> Self {
        Heading1Child::Progress(builder.build())
    }
}
impl From<Quote> for Heading1Child {
    fn from(child: Quote) -> Self {
        Heading1Child::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for Heading1Child {
    fn from(builder: builders::QuoteBuilder) -> Self {
        Heading1Child::Quote(builder.build())
    }
}
impl From<Ruby> for Heading1Child {
    fn from(child: Ruby) -> Self {
        Heading1Child::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for Heading1Child {
    fn from(builder: builders::RubyBuilder) -> Self {
        Heading1Child::Ruby(builder.build())
    }
}
impl From<Sample> for Heading1Child {
    fn from(child: Sample) -> Self {
        Heading1Child::Sample(child)
    }
}
impl From<builders::SampleBuilder> for Heading1Child {
    fn from(builder: builders::SampleBuilder) -> Self {
        Heading1Child::Sample(builder.build())
    }
}
impl From<Script> for Heading1Child {
    fn from(child: Script) -> Self {
        Heading1Child::Script(child)
    }
}
impl From<builders::ScriptBuilder> for Heading1Child {
    fn from(builder: builders::ScriptBuilder) -> Self {
        Heading1Child::Script(builder.build())
    }
}
impl From<Select> for Heading1Child {
    fn from(child: Select) -> Self {
        Heading1Child::Select(child)
    }
}
impl From<builders::SelectBuilder> for Heading1Child {
    fn from(builder: builders::SelectBuilder) -> Self {
        Heading1Child::Select(builder.build())
    }
}
impl From<Slot> for Heading1Child {
    fn from(child: Slot) -> Self {
        Heading1Child::Slot(child)
    }
}
impl From<builders::SlotBuilder> for Heading1Child {
    fn from(builder: builders::SlotBuilder) -> Self {
        Heading1Child::Slot(builder.build())
    }
}
impl From<Small> for Heading1Child {
    fn from(child: Small) -> Self {
        Heading1Child::Small(child)
    }
}
impl From<builders::SmallBuilder> for Heading1Child {
    fn from(builder: builders::SmallBuilder) -> Self {
        Heading1Child::Small(builder.build())
    }
}
impl From<Span> for Heading1Child {
    fn from(child: Span) -> Self {
        Heading1Child::Span(child)
    }
}
impl From<builders::SpanBuilder> for Heading1Child {
    fn from(builder: builders::SpanBuilder) -> Self {
        Heading1Child::Span(builder.build())
    }
}
impl From<StrikeThrough> for Heading1Child {
    fn from(child: StrikeThrough) -> Self {
        Heading1Child::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for Heading1Child {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        Heading1Child::StrikeThrough(builder.build())
    }
}
impl From<Strong> for Heading1Child {
    fn from(child: Strong) -> Self {
        Heading1Child::Strong(child)
    }
}
impl From<builders::StrongBuilder> for Heading1Child {
    fn from(builder: builders::StrongBuilder) -> Self {
        Heading1Child::Strong(builder.build())
    }
}
impl From<SubScript> for Heading1Child {
    fn from(child: SubScript) -> Self {
        Heading1Child::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for Heading1Child {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        Heading1Child::SubScript(builder.build())
    }
}
impl From<SupScript> for Heading1Child {
    fn from(child: SupScript) -> Self {
        Heading1Child::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for Heading1Child {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        Heading1Child::SupScript(builder.build())
    }
}
impl From<Template> for Heading1Child {
    fn from(child: Template) -> Self {
        Heading1Child::Template(child)
    }
}
impl From<builders::TemplateBuilder> for Heading1Child {
    fn from(builder: builders::TemplateBuilder) -> Self {
        Heading1Child::Template(builder.build())
    }
}
impl From<TextArea> for Heading1Child {
    fn from(child: TextArea) -> Self {
        Heading1Child::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for Heading1Child {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        Heading1Child::TextArea(builder.build())
    }
}
impl From<Time> for Heading1Child {
    fn from(child: Time) -> Self {
        Heading1Child::Time(child)
    }
}
impl From<builders::TimeBuilder> for Heading1Child {
    fn from(builder: builders::TimeBuilder) -> Self {
        Heading1Child::Time(builder.build())
    }
}
impl From<Underline> for Heading1Child {
    fn from(child: Underline) -> Self {
        Heading1Child::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for Heading1Child {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        Heading1Child::Underline(builder.build())
    }
}
impl From<Variable> for Heading1Child {
    fn from(child: Variable) -> Self {
        Heading1Child::Variable(child)
    }
}
impl From<builders::VariableBuilder> for Heading1Child {
    fn from(builder: builders::VariableBuilder) -> Self {
        Heading1Child::Variable(builder.build())
    }
}
impl From<Video> for Heading1Child {
    fn from(child: Video) -> Self {
        Heading1Child::Video(child)
    }
}
impl From<builders::VideoBuilder> for Heading1Child {
    fn from(builder: builders::VideoBuilder) -> Self {
        Heading1Child::Video(builder.build())
    }
}
impl From<WordBreak> for Heading1Child {
    fn from(child: WordBreak) -> Self {
        Heading1Child::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for Heading1Child {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        Heading1Child::WordBreak(builder.build())
    }
}
impl From<&'static str> for Heading1Child {
    fn from(s: &'static str) -> Self {
        Heading1Child::Text(s.into())
    }
}
impl From<String> for Heading1Child {
    fn from(s: String) -> Self {
        Heading1Child::Text(s.into())
    }
}
impl From<CowStr> for Heading1Child {
    fn from(s: CowStr) -> Self {
        Heading1Child::Text(s)
    }
}
impl std::fmt::Debug for Heading1Child {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heading1Child::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Anchor(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Audio(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Bold(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Button(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Canvas(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Cite(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Code(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Custom(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Data(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::DataList(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Definition(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Deleted(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Embed(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Image(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Input(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Inserted(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Italic(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Label(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Link(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Map(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::MapArea(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Mark(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Metadata(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Meter(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::NoScript(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Object(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Output(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Picture(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Progress(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Quote(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Ruby(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Sample(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Script(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Select(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Slot(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Small(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Span(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Strong(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::SubScript(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::SupScript(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Template(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::TextArea(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Time(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Underline(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Variable(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Video(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            Heading1Child::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for Heading1Child {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heading1Child::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Anchor(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Audio(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            Heading1Child::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            Heading1Child::Bold(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Button(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Canvas(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Cite(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Code(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Custom(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Data(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::DataList(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Definition(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Deleted(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Embed(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Emphasis(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Image(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Input(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Inserted(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Italic(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Keyboard(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Label(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::LineBreak(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Link(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Map(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::MapArea(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Mark(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Metadata(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Meter(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::NoScript(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Object(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Output(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Picture(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Progress(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Quote(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Ruby(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Sample(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Script(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Select(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Slot(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Small(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Span(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Strong(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::SubScript(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::SupScript(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Template(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::TextArea(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Time(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Underline(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Variable(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Video(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::WordBreak(child) => std::fmt::Display::fmt(child, f),
            Heading1Child::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
