// 🤖 This file is generated!

use crate::*;
/// The `<u>` element's children.
#[derive(Clone)]
pub enum UnderlineChild {
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
impl From<Abbreviation> for UnderlineChild {
    fn from(child: Abbreviation) -> Self {
        UnderlineChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for UnderlineChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        UnderlineChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for UnderlineChild {
    fn from(child: Anchor) -> Self {
        UnderlineChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for UnderlineChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        UnderlineChild::Anchor(builder.build())
    }
}
impl From<Audio> for UnderlineChild {
    fn from(child: Audio) -> Self {
        UnderlineChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for UnderlineChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        UnderlineChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for UnderlineChild {
    fn from(child: BidirectionalIsolate) -> Self {
        UnderlineChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for UnderlineChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        UnderlineChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for UnderlineChild {
    fn from(child: BidirectionalOverride) -> Self {
        UnderlineChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for UnderlineChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        UnderlineChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for UnderlineChild {
    fn from(child: Bold) -> Self {
        UnderlineChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for UnderlineChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        UnderlineChild::Bold(builder.build())
    }
}
impl From<Button> for UnderlineChild {
    fn from(child: Button) -> Self {
        UnderlineChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for UnderlineChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        UnderlineChild::Button(builder.build())
    }
}
impl From<Canvas> for UnderlineChild {
    fn from(child: Canvas) -> Self {
        UnderlineChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for UnderlineChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        UnderlineChild::Canvas(builder.build())
    }
}
impl From<Cite> for UnderlineChild {
    fn from(child: Cite) -> Self {
        UnderlineChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for UnderlineChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        UnderlineChild::Cite(builder.build())
    }
}
impl From<Code> for UnderlineChild {
    fn from(child: Code) -> Self {
        UnderlineChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for UnderlineChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        UnderlineChild::Code(builder.build())
    }
}
impl From<Custom> for UnderlineChild {
    fn from(child: Custom) -> Self {
        UnderlineChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for UnderlineChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        UnderlineChild::Custom(builder.build())
    }
}
impl From<Data> for UnderlineChild {
    fn from(child: Data) -> Self {
        UnderlineChild::Data(child)
    }
}
impl From<builders::DataBuilder> for UnderlineChild {
    fn from(builder: builders::DataBuilder) -> Self {
        UnderlineChild::Data(builder.build())
    }
}
impl From<DataList> for UnderlineChild {
    fn from(child: DataList) -> Self {
        UnderlineChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for UnderlineChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        UnderlineChild::DataList(builder.build())
    }
}
impl From<Definition> for UnderlineChild {
    fn from(child: Definition) -> Self {
        UnderlineChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for UnderlineChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        UnderlineChild::Definition(builder.build())
    }
}
impl From<Deleted> for UnderlineChild {
    fn from(child: Deleted) -> Self {
        UnderlineChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for UnderlineChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        UnderlineChild::Deleted(builder.build())
    }
}
impl From<Embed> for UnderlineChild {
    fn from(child: Embed) -> Self {
        UnderlineChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for UnderlineChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        UnderlineChild::Embed(builder.build())
    }
}
impl From<Emphasis> for UnderlineChild {
    fn from(child: Emphasis) -> Self {
        UnderlineChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for UnderlineChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        UnderlineChild::Emphasis(builder.build())
    }
}
impl From<Image> for UnderlineChild {
    fn from(child: Image) -> Self {
        UnderlineChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for UnderlineChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        UnderlineChild::Image(builder.build())
    }
}
impl From<InlineFrame> for UnderlineChild {
    fn from(child: InlineFrame) -> Self {
        UnderlineChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for UnderlineChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        UnderlineChild::InlineFrame(builder.build())
    }
}
impl From<Input> for UnderlineChild {
    fn from(child: Input) -> Self {
        UnderlineChild::Input(child)
    }
}
impl From<builders::InputBuilder> for UnderlineChild {
    fn from(builder: builders::InputBuilder) -> Self {
        UnderlineChild::Input(builder.build())
    }
}
impl From<Inserted> for UnderlineChild {
    fn from(child: Inserted) -> Self {
        UnderlineChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for UnderlineChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        UnderlineChild::Inserted(builder.build())
    }
}
impl From<Italic> for UnderlineChild {
    fn from(child: Italic) -> Self {
        UnderlineChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for UnderlineChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        UnderlineChild::Italic(builder.build())
    }
}
impl From<Keyboard> for UnderlineChild {
    fn from(child: Keyboard) -> Self {
        UnderlineChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for UnderlineChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        UnderlineChild::Keyboard(builder.build())
    }
}
impl From<Label> for UnderlineChild {
    fn from(child: Label) -> Self {
        UnderlineChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for UnderlineChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        UnderlineChild::Label(builder.build())
    }
}
impl From<LineBreak> for UnderlineChild {
    fn from(child: LineBreak) -> Self {
        UnderlineChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for UnderlineChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        UnderlineChild::LineBreak(builder.build())
    }
}
impl From<Link> for UnderlineChild {
    fn from(child: Link) -> Self {
        UnderlineChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for UnderlineChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        UnderlineChild::Link(builder.build())
    }
}
impl From<Map> for UnderlineChild {
    fn from(child: Map) -> Self {
        UnderlineChild::Map(child)
    }
}
impl From<builders::MapBuilder> for UnderlineChild {
    fn from(builder: builders::MapBuilder) -> Self {
        UnderlineChild::Map(builder.build())
    }
}
impl From<MapArea> for UnderlineChild {
    fn from(child: MapArea) -> Self {
        UnderlineChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for UnderlineChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        UnderlineChild::MapArea(builder.build())
    }
}
impl From<Mark> for UnderlineChild {
    fn from(child: Mark) -> Self {
        UnderlineChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for UnderlineChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        UnderlineChild::Mark(builder.build())
    }
}
impl From<Metadata> for UnderlineChild {
    fn from(child: Metadata) -> Self {
        UnderlineChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for UnderlineChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        UnderlineChild::Metadata(builder.build())
    }
}
impl From<Meter> for UnderlineChild {
    fn from(child: Meter) -> Self {
        UnderlineChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for UnderlineChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        UnderlineChild::Meter(builder.build())
    }
}
impl From<NoScript> for UnderlineChild {
    fn from(child: NoScript) -> Self {
        UnderlineChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for UnderlineChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        UnderlineChild::NoScript(builder.build())
    }
}
impl From<Object> for UnderlineChild {
    fn from(child: Object) -> Self {
        UnderlineChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for UnderlineChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        UnderlineChild::Object(builder.build())
    }
}
impl From<Output> for UnderlineChild {
    fn from(child: Output) -> Self {
        UnderlineChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for UnderlineChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        UnderlineChild::Output(builder.build())
    }
}
impl From<Picture> for UnderlineChild {
    fn from(child: Picture) -> Self {
        UnderlineChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for UnderlineChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        UnderlineChild::Picture(builder.build())
    }
}
impl From<Progress> for UnderlineChild {
    fn from(child: Progress) -> Self {
        UnderlineChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for UnderlineChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        UnderlineChild::Progress(builder.build())
    }
}
impl From<Quote> for UnderlineChild {
    fn from(child: Quote) -> Self {
        UnderlineChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for UnderlineChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        UnderlineChild::Quote(builder.build())
    }
}
impl From<Ruby> for UnderlineChild {
    fn from(child: Ruby) -> Self {
        UnderlineChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for UnderlineChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        UnderlineChild::Ruby(builder.build())
    }
}
impl From<Sample> for UnderlineChild {
    fn from(child: Sample) -> Self {
        UnderlineChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for UnderlineChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        UnderlineChild::Sample(builder.build())
    }
}
impl From<Script> for UnderlineChild {
    fn from(child: Script) -> Self {
        UnderlineChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for UnderlineChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        UnderlineChild::Script(builder.build())
    }
}
impl From<Select> for UnderlineChild {
    fn from(child: Select) -> Self {
        UnderlineChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for UnderlineChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        UnderlineChild::Select(builder.build())
    }
}
impl From<Slot> for UnderlineChild {
    fn from(child: Slot) -> Self {
        UnderlineChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for UnderlineChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        UnderlineChild::Slot(builder.build())
    }
}
impl From<Small> for UnderlineChild {
    fn from(child: Small) -> Self {
        UnderlineChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for UnderlineChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        UnderlineChild::Small(builder.build())
    }
}
impl From<Span> for UnderlineChild {
    fn from(child: Span) -> Self {
        UnderlineChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for UnderlineChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        UnderlineChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for UnderlineChild {
    fn from(child: StrikeThrough) -> Self {
        UnderlineChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for UnderlineChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        UnderlineChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for UnderlineChild {
    fn from(child: Strong) -> Self {
        UnderlineChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for UnderlineChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        UnderlineChild::Strong(builder.build())
    }
}
impl From<SubScript> for UnderlineChild {
    fn from(child: SubScript) -> Self {
        UnderlineChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for UnderlineChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        UnderlineChild::SubScript(builder.build())
    }
}
impl From<SupScript> for UnderlineChild {
    fn from(child: SupScript) -> Self {
        UnderlineChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for UnderlineChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        UnderlineChild::SupScript(builder.build())
    }
}
impl From<Template> for UnderlineChild {
    fn from(child: Template) -> Self {
        UnderlineChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for UnderlineChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        UnderlineChild::Template(builder.build())
    }
}
impl From<TextArea> for UnderlineChild {
    fn from(child: TextArea) -> Self {
        UnderlineChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for UnderlineChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        UnderlineChild::TextArea(builder.build())
    }
}
impl From<Time> for UnderlineChild {
    fn from(child: Time) -> Self {
        UnderlineChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for UnderlineChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        UnderlineChild::Time(builder.build())
    }
}
impl From<Underline> for UnderlineChild {
    fn from(child: Underline) -> Self {
        UnderlineChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for UnderlineChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        UnderlineChild::Underline(builder.build())
    }
}
impl From<Variable> for UnderlineChild {
    fn from(child: Variable) -> Self {
        UnderlineChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for UnderlineChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        UnderlineChild::Variable(builder.build())
    }
}
impl From<Video> for UnderlineChild {
    fn from(child: Video) -> Self {
        UnderlineChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for UnderlineChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        UnderlineChild::Video(builder.build())
    }
}
impl From<WordBreak> for UnderlineChild {
    fn from(child: WordBreak) -> Self {
        UnderlineChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for UnderlineChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        UnderlineChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for UnderlineChild {
    fn from(s: &'static str) -> Self {
        UnderlineChild::Text(s.into())
    }
}
impl From<String> for UnderlineChild {
    fn from(s: String) -> Self {
        UnderlineChild::Text(s.into())
    }
}
impl From<CowStr> for UnderlineChild {
    fn from(s: CowStr) -> Self {
        UnderlineChild::Text(s)
    }
}
impl std::fmt::Debug for UnderlineChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnderlineChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            UnderlineChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Button(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Code(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Data(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Image(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Input(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Label(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Link(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Map(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Object(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Output(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Script(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Select(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Small(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Span(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Template(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Time(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Video(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            UnderlineChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for UnderlineChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnderlineChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Audio(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            UnderlineChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            UnderlineChild::Bold(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Button(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Cite(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Code(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Custom(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Data(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::DataList(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Definition(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Embed(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Image(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Input(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Italic(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Label(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Link(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Map(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Mark(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Meter(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Object(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Output(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Picture(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Progress(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Quote(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Sample(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Script(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Select(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Slot(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Small(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Span(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Strong(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Template(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Time(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Underline(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Variable(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Video(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            UnderlineChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
