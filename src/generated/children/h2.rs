// 🤖 This file is generated!

use crate::*;
/// The `<h2>` element's children.
#[derive(Clone)]
pub enum Heading2Child {
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
impl From<Abbreviation> for Heading2Child {
    fn from(child: Abbreviation) -> Self {
        Heading2Child::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for Heading2Child {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        Heading2Child::Abbreviation(builder.build())
    }
}
impl From<Anchor> for Heading2Child {
    fn from(child: Anchor) -> Self {
        Heading2Child::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for Heading2Child {
    fn from(builder: builders::AnchorBuilder) -> Self {
        Heading2Child::Anchor(builder.build())
    }
}
impl From<Audio> for Heading2Child {
    fn from(child: Audio) -> Self {
        Heading2Child::Audio(child)
    }
}
impl From<builders::AudioBuilder> for Heading2Child {
    fn from(builder: builders::AudioBuilder) -> Self {
        Heading2Child::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for Heading2Child {
    fn from(child: BidirectionalIsolate) -> Self {
        Heading2Child::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for Heading2Child {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        Heading2Child::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for Heading2Child {
    fn from(child: BidirectionalOverride) -> Self {
        Heading2Child::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for Heading2Child {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        Heading2Child::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for Heading2Child {
    fn from(child: Bold) -> Self {
        Heading2Child::Bold(child)
    }
}
impl From<builders::BoldBuilder> for Heading2Child {
    fn from(builder: builders::BoldBuilder) -> Self {
        Heading2Child::Bold(builder.build())
    }
}
impl From<Button> for Heading2Child {
    fn from(child: Button) -> Self {
        Heading2Child::Button(child)
    }
}
impl From<builders::ButtonBuilder> for Heading2Child {
    fn from(builder: builders::ButtonBuilder) -> Self {
        Heading2Child::Button(builder.build())
    }
}
impl From<Canvas> for Heading2Child {
    fn from(child: Canvas) -> Self {
        Heading2Child::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for Heading2Child {
    fn from(builder: builders::CanvasBuilder) -> Self {
        Heading2Child::Canvas(builder.build())
    }
}
impl From<Cite> for Heading2Child {
    fn from(child: Cite) -> Self {
        Heading2Child::Cite(child)
    }
}
impl From<builders::CiteBuilder> for Heading2Child {
    fn from(builder: builders::CiteBuilder) -> Self {
        Heading2Child::Cite(builder.build())
    }
}
impl From<Code> for Heading2Child {
    fn from(child: Code) -> Self {
        Heading2Child::Code(child)
    }
}
impl From<builders::CodeBuilder> for Heading2Child {
    fn from(builder: builders::CodeBuilder) -> Self {
        Heading2Child::Code(builder.build())
    }
}
impl From<Custom> for Heading2Child {
    fn from(child: Custom) -> Self {
        Heading2Child::Custom(child)
    }
}
impl From<builders::CustomBuilder> for Heading2Child {
    fn from(builder: builders::CustomBuilder) -> Self {
        Heading2Child::Custom(builder.build())
    }
}
impl From<Data> for Heading2Child {
    fn from(child: Data) -> Self {
        Heading2Child::Data(child)
    }
}
impl From<builders::DataBuilder> for Heading2Child {
    fn from(builder: builders::DataBuilder) -> Self {
        Heading2Child::Data(builder.build())
    }
}
impl From<DataList> for Heading2Child {
    fn from(child: DataList) -> Self {
        Heading2Child::DataList(child)
    }
}
impl From<builders::DataListBuilder> for Heading2Child {
    fn from(builder: builders::DataListBuilder) -> Self {
        Heading2Child::DataList(builder.build())
    }
}
impl From<Definition> for Heading2Child {
    fn from(child: Definition) -> Self {
        Heading2Child::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for Heading2Child {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        Heading2Child::Definition(builder.build())
    }
}
impl From<Deleted> for Heading2Child {
    fn from(child: Deleted) -> Self {
        Heading2Child::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for Heading2Child {
    fn from(builder: builders::DeletedBuilder) -> Self {
        Heading2Child::Deleted(builder.build())
    }
}
impl From<Embed> for Heading2Child {
    fn from(child: Embed) -> Self {
        Heading2Child::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for Heading2Child {
    fn from(builder: builders::EmbedBuilder) -> Self {
        Heading2Child::Embed(builder.build())
    }
}
impl From<Emphasis> for Heading2Child {
    fn from(child: Emphasis) -> Self {
        Heading2Child::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for Heading2Child {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        Heading2Child::Emphasis(builder.build())
    }
}
impl From<Image> for Heading2Child {
    fn from(child: Image) -> Self {
        Heading2Child::Image(child)
    }
}
impl From<builders::ImageBuilder> for Heading2Child {
    fn from(builder: builders::ImageBuilder) -> Self {
        Heading2Child::Image(builder.build())
    }
}
impl From<InlineFrame> for Heading2Child {
    fn from(child: InlineFrame) -> Self {
        Heading2Child::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for Heading2Child {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        Heading2Child::InlineFrame(builder.build())
    }
}
impl From<Input> for Heading2Child {
    fn from(child: Input) -> Self {
        Heading2Child::Input(child)
    }
}
impl From<builders::InputBuilder> for Heading2Child {
    fn from(builder: builders::InputBuilder) -> Self {
        Heading2Child::Input(builder.build())
    }
}
impl From<Inserted> for Heading2Child {
    fn from(child: Inserted) -> Self {
        Heading2Child::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for Heading2Child {
    fn from(builder: builders::InsertedBuilder) -> Self {
        Heading2Child::Inserted(builder.build())
    }
}
impl From<Italic> for Heading2Child {
    fn from(child: Italic) -> Self {
        Heading2Child::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for Heading2Child {
    fn from(builder: builders::ItalicBuilder) -> Self {
        Heading2Child::Italic(builder.build())
    }
}
impl From<Keyboard> for Heading2Child {
    fn from(child: Keyboard) -> Self {
        Heading2Child::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for Heading2Child {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        Heading2Child::Keyboard(builder.build())
    }
}
impl From<Label> for Heading2Child {
    fn from(child: Label) -> Self {
        Heading2Child::Label(child)
    }
}
impl From<builders::LabelBuilder> for Heading2Child {
    fn from(builder: builders::LabelBuilder) -> Self {
        Heading2Child::Label(builder.build())
    }
}
impl From<LineBreak> for Heading2Child {
    fn from(child: LineBreak) -> Self {
        Heading2Child::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for Heading2Child {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        Heading2Child::LineBreak(builder.build())
    }
}
impl From<Link> for Heading2Child {
    fn from(child: Link) -> Self {
        Heading2Child::Link(child)
    }
}
impl From<builders::LinkBuilder> for Heading2Child {
    fn from(builder: builders::LinkBuilder) -> Self {
        Heading2Child::Link(builder.build())
    }
}
impl From<Map> for Heading2Child {
    fn from(child: Map) -> Self {
        Heading2Child::Map(child)
    }
}
impl From<builders::MapBuilder> for Heading2Child {
    fn from(builder: builders::MapBuilder) -> Self {
        Heading2Child::Map(builder.build())
    }
}
impl From<MapArea> for Heading2Child {
    fn from(child: MapArea) -> Self {
        Heading2Child::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for Heading2Child {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        Heading2Child::MapArea(builder.build())
    }
}
impl From<Mark> for Heading2Child {
    fn from(child: Mark) -> Self {
        Heading2Child::Mark(child)
    }
}
impl From<builders::MarkBuilder> for Heading2Child {
    fn from(builder: builders::MarkBuilder) -> Self {
        Heading2Child::Mark(builder.build())
    }
}
impl From<Metadata> for Heading2Child {
    fn from(child: Metadata) -> Self {
        Heading2Child::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for Heading2Child {
    fn from(builder: builders::MetadataBuilder) -> Self {
        Heading2Child::Metadata(builder.build())
    }
}
impl From<Meter> for Heading2Child {
    fn from(child: Meter) -> Self {
        Heading2Child::Meter(child)
    }
}
impl From<builders::MeterBuilder> for Heading2Child {
    fn from(builder: builders::MeterBuilder) -> Self {
        Heading2Child::Meter(builder.build())
    }
}
impl From<NoScript> for Heading2Child {
    fn from(child: NoScript) -> Self {
        Heading2Child::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for Heading2Child {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        Heading2Child::NoScript(builder.build())
    }
}
impl From<Object> for Heading2Child {
    fn from(child: Object) -> Self {
        Heading2Child::Object(child)
    }
}
impl From<builders::ObjectBuilder> for Heading2Child {
    fn from(builder: builders::ObjectBuilder) -> Self {
        Heading2Child::Object(builder.build())
    }
}
impl From<Output> for Heading2Child {
    fn from(child: Output) -> Self {
        Heading2Child::Output(child)
    }
}
impl From<builders::OutputBuilder> for Heading2Child {
    fn from(builder: builders::OutputBuilder) -> Self {
        Heading2Child::Output(builder.build())
    }
}
impl From<Picture> for Heading2Child {
    fn from(child: Picture) -> Self {
        Heading2Child::Picture(child)
    }
}
impl From<builders::PictureBuilder> for Heading2Child {
    fn from(builder: builders::PictureBuilder) -> Self {
        Heading2Child::Picture(builder.build())
    }
}
impl From<Progress> for Heading2Child {
    fn from(child: Progress) -> Self {
        Heading2Child::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for Heading2Child {
    fn from(builder: builders::ProgressBuilder) -> Self {
        Heading2Child::Progress(builder.build())
    }
}
impl From<Quote> for Heading2Child {
    fn from(child: Quote) -> Self {
        Heading2Child::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for Heading2Child {
    fn from(builder: builders::QuoteBuilder) -> Self {
        Heading2Child::Quote(builder.build())
    }
}
impl From<Ruby> for Heading2Child {
    fn from(child: Ruby) -> Self {
        Heading2Child::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for Heading2Child {
    fn from(builder: builders::RubyBuilder) -> Self {
        Heading2Child::Ruby(builder.build())
    }
}
impl From<Sample> for Heading2Child {
    fn from(child: Sample) -> Self {
        Heading2Child::Sample(child)
    }
}
impl From<builders::SampleBuilder> for Heading2Child {
    fn from(builder: builders::SampleBuilder) -> Self {
        Heading2Child::Sample(builder.build())
    }
}
impl From<Script> for Heading2Child {
    fn from(child: Script) -> Self {
        Heading2Child::Script(child)
    }
}
impl From<builders::ScriptBuilder> for Heading2Child {
    fn from(builder: builders::ScriptBuilder) -> Self {
        Heading2Child::Script(builder.build())
    }
}
impl From<Select> for Heading2Child {
    fn from(child: Select) -> Self {
        Heading2Child::Select(child)
    }
}
impl From<builders::SelectBuilder> for Heading2Child {
    fn from(builder: builders::SelectBuilder) -> Self {
        Heading2Child::Select(builder.build())
    }
}
impl From<Slot> for Heading2Child {
    fn from(child: Slot) -> Self {
        Heading2Child::Slot(child)
    }
}
impl From<builders::SlotBuilder> for Heading2Child {
    fn from(builder: builders::SlotBuilder) -> Self {
        Heading2Child::Slot(builder.build())
    }
}
impl From<Small> for Heading2Child {
    fn from(child: Small) -> Self {
        Heading2Child::Small(child)
    }
}
impl From<builders::SmallBuilder> for Heading2Child {
    fn from(builder: builders::SmallBuilder) -> Self {
        Heading2Child::Small(builder.build())
    }
}
impl From<Span> for Heading2Child {
    fn from(child: Span) -> Self {
        Heading2Child::Span(child)
    }
}
impl From<builders::SpanBuilder> for Heading2Child {
    fn from(builder: builders::SpanBuilder) -> Self {
        Heading2Child::Span(builder.build())
    }
}
impl From<StrikeThrough> for Heading2Child {
    fn from(child: StrikeThrough) -> Self {
        Heading2Child::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for Heading2Child {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        Heading2Child::StrikeThrough(builder.build())
    }
}
impl From<Strong> for Heading2Child {
    fn from(child: Strong) -> Self {
        Heading2Child::Strong(child)
    }
}
impl From<builders::StrongBuilder> for Heading2Child {
    fn from(builder: builders::StrongBuilder) -> Self {
        Heading2Child::Strong(builder.build())
    }
}
impl From<SubScript> for Heading2Child {
    fn from(child: SubScript) -> Self {
        Heading2Child::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for Heading2Child {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        Heading2Child::SubScript(builder.build())
    }
}
impl From<SupScript> for Heading2Child {
    fn from(child: SupScript) -> Self {
        Heading2Child::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for Heading2Child {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        Heading2Child::SupScript(builder.build())
    }
}
impl From<Template> for Heading2Child {
    fn from(child: Template) -> Self {
        Heading2Child::Template(child)
    }
}
impl From<builders::TemplateBuilder> for Heading2Child {
    fn from(builder: builders::TemplateBuilder) -> Self {
        Heading2Child::Template(builder.build())
    }
}
impl From<TextArea> for Heading2Child {
    fn from(child: TextArea) -> Self {
        Heading2Child::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for Heading2Child {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        Heading2Child::TextArea(builder.build())
    }
}
impl From<Time> for Heading2Child {
    fn from(child: Time) -> Self {
        Heading2Child::Time(child)
    }
}
impl From<builders::TimeBuilder> for Heading2Child {
    fn from(builder: builders::TimeBuilder) -> Self {
        Heading2Child::Time(builder.build())
    }
}
impl From<Underline> for Heading2Child {
    fn from(child: Underline) -> Self {
        Heading2Child::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for Heading2Child {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        Heading2Child::Underline(builder.build())
    }
}
impl From<Variable> for Heading2Child {
    fn from(child: Variable) -> Self {
        Heading2Child::Variable(child)
    }
}
impl From<builders::VariableBuilder> for Heading2Child {
    fn from(builder: builders::VariableBuilder) -> Self {
        Heading2Child::Variable(builder.build())
    }
}
impl From<Video> for Heading2Child {
    fn from(child: Video) -> Self {
        Heading2Child::Video(child)
    }
}
impl From<builders::VideoBuilder> for Heading2Child {
    fn from(builder: builders::VideoBuilder) -> Self {
        Heading2Child::Video(builder.build())
    }
}
impl From<WordBreak> for Heading2Child {
    fn from(child: WordBreak) -> Self {
        Heading2Child::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for Heading2Child {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        Heading2Child::WordBreak(builder.build())
    }
}
impl From<&'static str> for Heading2Child {
    fn from(s: &'static str) -> Self {
        Heading2Child::Text(s.into())
    }
}
impl From<String> for Heading2Child {
    fn from(s: String) -> Self {
        Heading2Child::Text(s.into())
    }
}
impl From<CowStr> for Heading2Child {
    fn from(s: CowStr) -> Self {
        Heading2Child::Text(s)
    }
}
impl std::fmt::Debug for Heading2Child {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heading2Child::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Anchor(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Audio(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Bold(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Button(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Canvas(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Cite(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Code(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Custom(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Data(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::DataList(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Definition(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Deleted(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Embed(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Image(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Input(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Inserted(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Italic(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Label(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Link(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Map(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::MapArea(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Mark(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Metadata(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Meter(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::NoScript(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Object(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Output(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Picture(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Progress(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Quote(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Ruby(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Sample(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Script(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Select(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Slot(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Small(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Span(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Strong(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::SubScript(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::SupScript(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Template(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::TextArea(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Time(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Underline(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Variable(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Video(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            Heading2Child::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for Heading2Child {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heading2Child::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Anchor(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Audio(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            Heading2Child::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            Heading2Child::Bold(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Button(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Canvas(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Cite(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Code(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Custom(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Data(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::DataList(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Definition(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Deleted(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Embed(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Emphasis(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Image(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Input(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Inserted(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Italic(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Keyboard(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Label(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::LineBreak(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Link(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Map(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::MapArea(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Mark(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Metadata(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Meter(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::NoScript(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Object(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Output(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Picture(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Progress(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Quote(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Ruby(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Sample(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Script(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Select(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Slot(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Small(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Span(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Strong(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::SubScript(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::SupScript(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Template(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::TextArea(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Time(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Underline(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Variable(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Video(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::WordBreak(child) => std::fmt::Display::fmt(child, f),
            Heading2Child::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
