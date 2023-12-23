// 🤖 This file is generated!

use crate::*;
/// The `<h6>` element's children.
#[derive(Clone)]
pub enum Heading6Child {
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
impl From<Abbreviation> for Heading6Child {
    fn from(child: Abbreviation) -> Self {
        Heading6Child::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for Heading6Child {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        Heading6Child::Abbreviation(builder.build())
    }
}
impl From<Anchor> for Heading6Child {
    fn from(child: Anchor) -> Self {
        Heading6Child::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for Heading6Child {
    fn from(builder: builders::AnchorBuilder) -> Self {
        Heading6Child::Anchor(builder.build())
    }
}
impl From<Audio> for Heading6Child {
    fn from(child: Audio) -> Self {
        Heading6Child::Audio(child)
    }
}
impl From<builders::AudioBuilder> for Heading6Child {
    fn from(builder: builders::AudioBuilder) -> Self {
        Heading6Child::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for Heading6Child {
    fn from(child: BidirectionalIsolate) -> Self {
        Heading6Child::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for Heading6Child {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        Heading6Child::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for Heading6Child {
    fn from(child: BidirectionalOverride) -> Self {
        Heading6Child::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for Heading6Child {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        Heading6Child::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for Heading6Child {
    fn from(child: Bold) -> Self {
        Heading6Child::Bold(child)
    }
}
impl From<builders::BoldBuilder> for Heading6Child {
    fn from(builder: builders::BoldBuilder) -> Self {
        Heading6Child::Bold(builder.build())
    }
}
impl From<Button> for Heading6Child {
    fn from(child: Button) -> Self {
        Heading6Child::Button(child)
    }
}
impl From<builders::ButtonBuilder> for Heading6Child {
    fn from(builder: builders::ButtonBuilder) -> Self {
        Heading6Child::Button(builder.build())
    }
}
impl From<Canvas> for Heading6Child {
    fn from(child: Canvas) -> Self {
        Heading6Child::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for Heading6Child {
    fn from(builder: builders::CanvasBuilder) -> Self {
        Heading6Child::Canvas(builder.build())
    }
}
impl From<Cite> for Heading6Child {
    fn from(child: Cite) -> Self {
        Heading6Child::Cite(child)
    }
}
impl From<builders::CiteBuilder> for Heading6Child {
    fn from(builder: builders::CiteBuilder) -> Self {
        Heading6Child::Cite(builder.build())
    }
}
impl From<Code> for Heading6Child {
    fn from(child: Code) -> Self {
        Heading6Child::Code(child)
    }
}
impl From<builders::CodeBuilder> for Heading6Child {
    fn from(builder: builders::CodeBuilder) -> Self {
        Heading6Child::Code(builder.build())
    }
}
impl From<Custom> for Heading6Child {
    fn from(child: Custom) -> Self {
        Heading6Child::Custom(child)
    }
}
impl From<builders::CustomBuilder> for Heading6Child {
    fn from(builder: builders::CustomBuilder) -> Self {
        Heading6Child::Custom(builder.build())
    }
}
impl From<Data> for Heading6Child {
    fn from(child: Data) -> Self {
        Heading6Child::Data(child)
    }
}
impl From<builders::DataBuilder> for Heading6Child {
    fn from(builder: builders::DataBuilder) -> Self {
        Heading6Child::Data(builder.build())
    }
}
impl From<DataList> for Heading6Child {
    fn from(child: DataList) -> Self {
        Heading6Child::DataList(child)
    }
}
impl From<builders::DataListBuilder> for Heading6Child {
    fn from(builder: builders::DataListBuilder) -> Self {
        Heading6Child::DataList(builder.build())
    }
}
impl From<Definition> for Heading6Child {
    fn from(child: Definition) -> Self {
        Heading6Child::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for Heading6Child {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        Heading6Child::Definition(builder.build())
    }
}
impl From<Deleted> for Heading6Child {
    fn from(child: Deleted) -> Self {
        Heading6Child::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for Heading6Child {
    fn from(builder: builders::DeletedBuilder) -> Self {
        Heading6Child::Deleted(builder.build())
    }
}
impl From<Embed> for Heading6Child {
    fn from(child: Embed) -> Self {
        Heading6Child::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for Heading6Child {
    fn from(builder: builders::EmbedBuilder) -> Self {
        Heading6Child::Embed(builder.build())
    }
}
impl From<Emphasis> for Heading6Child {
    fn from(child: Emphasis) -> Self {
        Heading6Child::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for Heading6Child {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        Heading6Child::Emphasis(builder.build())
    }
}
impl From<Image> for Heading6Child {
    fn from(child: Image) -> Self {
        Heading6Child::Image(child)
    }
}
impl From<builders::ImageBuilder> for Heading6Child {
    fn from(builder: builders::ImageBuilder) -> Self {
        Heading6Child::Image(builder.build())
    }
}
impl From<InlineFrame> for Heading6Child {
    fn from(child: InlineFrame) -> Self {
        Heading6Child::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for Heading6Child {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        Heading6Child::InlineFrame(builder.build())
    }
}
impl From<Input> for Heading6Child {
    fn from(child: Input) -> Self {
        Heading6Child::Input(child)
    }
}
impl From<builders::InputBuilder> for Heading6Child {
    fn from(builder: builders::InputBuilder) -> Self {
        Heading6Child::Input(builder.build())
    }
}
impl From<Inserted> for Heading6Child {
    fn from(child: Inserted) -> Self {
        Heading6Child::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for Heading6Child {
    fn from(builder: builders::InsertedBuilder) -> Self {
        Heading6Child::Inserted(builder.build())
    }
}
impl From<Italic> for Heading6Child {
    fn from(child: Italic) -> Self {
        Heading6Child::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for Heading6Child {
    fn from(builder: builders::ItalicBuilder) -> Self {
        Heading6Child::Italic(builder.build())
    }
}
impl From<Keyboard> for Heading6Child {
    fn from(child: Keyboard) -> Self {
        Heading6Child::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for Heading6Child {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        Heading6Child::Keyboard(builder.build())
    }
}
impl From<Label> for Heading6Child {
    fn from(child: Label) -> Self {
        Heading6Child::Label(child)
    }
}
impl From<builders::LabelBuilder> for Heading6Child {
    fn from(builder: builders::LabelBuilder) -> Self {
        Heading6Child::Label(builder.build())
    }
}
impl From<LineBreak> for Heading6Child {
    fn from(child: LineBreak) -> Self {
        Heading6Child::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for Heading6Child {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        Heading6Child::LineBreak(builder.build())
    }
}
impl From<Link> for Heading6Child {
    fn from(child: Link) -> Self {
        Heading6Child::Link(child)
    }
}
impl From<builders::LinkBuilder> for Heading6Child {
    fn from(builder: builders::LinkBuilder) -> Self {
        Heading6Child::Link(builder.build())
    }
}
impl From<Map> for Heading6Child {
    fn from(child: Map) -> Self {
        Heading6Child::Map(child)
    }
}
impl From<builders::MapBuilder> for Heading6Child {
    fn from(builder: builders::MapBuilder) -> Self {
        Heading6Child::Map(builder.build())
    }
}
impl From<MapArea> for Heading6Child {
    fn from(child: MapArea) -> Self {
        Heading6Child::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for Heading6Child {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        Heading6Child::MapArea(builder.build())
    }
}
impl From<Mark> for Heading6Child {
    fn from(child: Mark) -> Self {
        Heading6Child::Mark(child)
    }
}
impl From<builders::MarkBuilder> for Heading6Child {
    fn from(builder: builders::MarkBuilder) -> Self {
        Heading6Child::Mark(builder.build())
    }
}
impl From<Metadata> for Heading6Child {
    fn from(child: Metadata) -> Self {
        Heading6Child::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for Heading6Child {
    fn from(builder: builders::MetadataBuilder) -> Self {
        Heading6Child::Metadata(builder.build())
    }
}
impl From<Meter> for Heading6Child {
    fn from(child: Meter) -> Self {
        Heading6Child::Meter(child)
    }
}
impl From<builders::MeterBuilder> for Heading6Child {
    fn from(builder: builders::MeterBuilder) -> Self {
        Heading6Child::Meter(builder.build())
    }
}
impl From<NoScript> for Heading6Child {
    fn from(child: NoScript) -> Self {
        Heading6Child::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for Heading6Child {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        Heading6Child::NoScript(builder.build())
    }
}
impl From<Object> for Heading6Child {
    fn from(child: Object) -> Self {
        Heading6Child::Object(child)
    }
}
impl From<builders::ObjectBuilder> for Heading6Child {
    fn from(builder: builders::ObjectBuilder) -> Self {
        Heading6Child::Object(builder.build())
    }
}
impl From<Output> for Heading6Child {
    fn from(child: Output) -> Self {
        Heading6Child::Output(child)
    }
}
impl From<builders::OutputBuilder> for Heading6Child {
    fn from(builder: builders::OutputBuilder) -> Self {
        Heading6Child::Output(builder.build())
    }
}
impl From<Picture> for Heading6Child {
    fn from(child: Picture) -> Self {
        Heading6Child::Picture(child)
    }
}
impl From<builders::PictureBuilder> for Heading6Child {
    fn from(builder: builders::PictureBuilder) -> Self {
        Heading6Child::Picture(builder.build())
    }
}
impl From<Progress> for Heading6Child {
    fn from(child: Progress) -> Self {
        Heading6Child::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for Heading6Child {
    fn from(builder: builders::ProgressBuilder) -> Self {
        Heading6Child::Progress(builder.build())
    }
}
impl From<Quote> for Heading6Child {
    fn from(child: Quote) -> Self {
        Heading6Child::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for Heading6Child {
    fn from(builder: builders::QuoteBuilder) -> Self {
        Heading6Child::Quote(builder.build())
    }
}
impl From<Ruby> for Heading6Child {
    fn from(child: Ruby) -> Self {
        Heading6Child::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for Heading6Child {
    fn from(builder: builders::RubyBuilder) -> Self {
        Heading6Child::Ruby(builder.build())
    }
}
impl From<Sample> for Heading6Child {
    fn from(child: Sample) -> Self {
        Heading6Child::Sample(child)
    }
}
impl From<builders::SampleBuilder> for Heading6Child {
    fn from(builder: builders::SampleBuilder) -> Self {
        Heading6Child::Sample(builder.build())
    }
}
impl From<Script> for Heading6Child {
    fn from(child: Script) -> Self {
        Heading6Child::Script(child)
    }
}
impl From<builders::ScriptBuilder> for Heading6Child {
    fn from(builder: builders::ScriptBuilder) -> Self {
        Heading6Child::Script(builder.build())
    }
}
impl From<Select> for Heading6Child {
    fn from(child: Select) -> Self {
        Heading6Child::Select(child)
    }
}
impl From<builders::SelectBuilder> for Heading6Child {
    fn from(builder: builders::SelectBuilder) -> Self {
        Heading6Child::Select(builder.build())
    }
}
impl From<Slot> for Heading6Child {
    fn from(child: Slot) -> Self {
        Heading6Child::Slot(child)
    }
}
impl From<builders::SlotBuilder> for Heading6Child {
    fn from(builder: builders::SlotBuilder) -> Self {
        Heading6Child::Slot(builder.build())
    }
}
impl From<Small> for Heading6Child {
    fn from(child: Small) -> Self {
        Heading6Child::Small(child)
    }
}
impl From<builders::SmallBuilder> for Heading6Child {
    fn from(builder: builders::SmallBuilder) -> Self {
        Heading6Child::Small(builder.build())
    }
}
impl From<Span> for Heading6Child {
    fn from(child: Span) -> Self {
        Heading6Child::Span(child)
    }
}
impl From<builders::SpanBuilder> for Heading6Child {
    fn from(builder: builders::SpanBuilder) -> Self {
        Heading6Child::Span(builder.build())
    }
}
impl From<StrikeThrough> for Heading6Child {
    fn from(child: StrikeThrough) -> Self {
        Heading6Child::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for Heading6Child {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        Heading6Child::StrikeThrough(builder.build())
    }
}
impl From<Strong> for Heading6Child {
    fn from(child: Strong) -> Self {
        Heading6Child::Strong(child)
    }
}
impl From<builders::StrongBuilder> for Heading6Child {
    fn from(builder: builders::StrongBuilder) -> Self {
        Heading6Child::Strong(builder.build())
    }
}
impl From<SubScript> for Heading6Child {
    fn from(child: SubScript) -> Self {
        Heading6Child::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for Heading6Child {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        Heading6Child::SubScript(builder.build())
    }
}
impl From<SupScript> for Heading6Child {
    fn from(child: SupScript) -> Self {
        Heading6Child::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for Heading6Child {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        Heading6Child::SupScript(builder.build())
    }
}
impl From<Template> for Heading6Child {
    fn from(child: Template) -> Self {
        Heading6Child::Template(child)
    }
}
impl From<builders::TemplateBuilder> for Heading6Child {
    fn from(builder: builders::TemplateBuilder) -> Self {
        Heading6Child::Template(builder.build())
    }
}
impl From<TextArea> for Heading6Child {
    fn from(child: TextArea) -> Self {
        Heading6Child::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for Heading6Child {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        Heading6Child::TextArea(builder.build())
    }
}
impl From<Time> for Heading6Child {
    fn from(child: Time) -> Self {
        Heading6Child::Time(child)
    }
}
impl From<builders::TimeBuilder> for Heading6Child {
    fn from(builder: builders::TimeBuilder) -> Self {
        Heading6Child::Time(builder.build())
    }
}
impl From<Underline> for Heading6Child {
    fn from(child: Underline) -> Self {
        Heading6Child::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for Heading6Child {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        Heading6Child::Underline(builder.build())
    }
}
impl From<Variable> for Heading6Child {
    fn from(child: Variable) -> Self {
        Heading6Child::Variable(child)
    }
}
impl From<builders::VariableBuilder> for Heading6Child {
    fn from(builder: builders::VariableBuilder) -> Self {
        Heading6Child::Variable(builder.build())
    }
}
impl From<Video> for Heading6Child {
    fn from(child: Video) -> Self {
        Heading6Child::Video(child)
    }
}
impl From<builders::VideoBuilder> for Heading6Child {
    fn from(builder: builders::VideoBuilder) -> Self {
        Heading6Child::Video(builder.build())
    }
}
impl From<WordBreak> for Heading6Child {
    fn from(child: WordBreak) -> Self {
        Heading6Child::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for Heading6Child {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        Heading6Child::WordBreak(builder.build())
    }
}
impl From<&'static str> for Heading6Child {
    fn from(s: &'static str) -> Self {
        Heading6Child::Text(s.into())
    }
}
impl From<String> for Heading6Child {
    fn from(s: String) -> Self {
        Heading6Child::Text(s.into())
    }
}
impl From<CowStr> for Heading6Child {
    fn from(s: CowStr) -> Self {
        Heading6Child::Text(s)
    }
}
impl std::fmt::Debug for Heading6Child {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heading6Child::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Anchor(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Audio(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Bold(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Button(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Canvas(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Cite(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Code(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Custom(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Data(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::DataList(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Definition(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Deleted(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Embed(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Image(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Input(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Inserted(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Italic(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Label(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Link(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Map(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::MapArea(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Mark(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Metadata(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Meter(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::NoScript(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Object(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Output(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Picture(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Progress(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Quote(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Ruby(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Sample(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Script(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Select(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Slot(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Small(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Span(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Strong(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::SubScript(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::SupScript(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Template(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::TextArea(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Time(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Underline(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Variable(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Video(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            Heading6Child::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for Heading6Child {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heading6Child::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Anchor(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Audio(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            Heading6Child::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            Heading6Child::Bold(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Button(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Canvas(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Cite(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Code(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Custom(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Data(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::DataList(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Definition(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Deleted(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Embed(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Emphasis(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Image(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Input(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Inserted(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Italic(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Keyboard(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Label(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::LineBreak(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Link(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Map(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::MapArea(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Mark(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Metadata(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Meter(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::NoScript(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Object(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Output(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Picture(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Progress(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Quote(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Ruby(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Sample(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Script(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Select(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Slot(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Small(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Span(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Strong(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::SubScript(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::SupScript(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Template(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::TextArea(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Time(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Underline(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Variable(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Video(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::WordBreak(child) => std::fmt::Display::fmt(child, f),
            Heading6Child::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
