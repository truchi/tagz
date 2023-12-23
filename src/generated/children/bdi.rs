// 🤖 This file is generated!

use crate::*;
/// The `<bdi>` element's children.
#[derive(Clone)]
pub enum BidirectionalIsolateChild {
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
impl From<Abbreviation> for BidirectionalIsolateChild {
    fn from(child: Abbreviation) -> Self {
        BidirectionalIsolateChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        BidirectionalIsolateChild::Abbreviation(builder.build())
    }
}
impl From<Anchor> for BidirectionalIsolateChild {
    fn from(child: Anchor) -> Self {
        BidirectionalIsolateChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        BidirectionalIsolateChild::Anchor(builder.build())
    }
}
impl From<Audio> for BidirectionalIsolateChild {
    fn from(child: Audio) -> Self {
        BidirectionalIsolateChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        BidirectionalIsolateChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for BidirectionalIsolateChild {
    fn from(child: BidirectionalIsolate) -> Self {
        BidirectionalIsolateChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        BidirectionalIsolateChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for BidirectionalIsolateChild {
    fn from(child: BidirectionalOverride) -> Self {
        BidirectionalIsolateChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        BidirectionalIsolateChild::BidirectionalOverride(builder.build())
    }
}
impl From<Bold> for BidirectionalIsolateChild {
    fn from(child: Bold) -> Self {
        BidirectionalIsolateChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        BidirectionalIsolateChild::Bold(builder.build())
    }
}
impl From<Button> for BidirectionalIsolateChild {
    fn from(child: Button) -> Self {
        BidirectionalIsolateChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        BidirectionalIsolateChild::Button(builder.build())
    }
}
impl From<Canvas> for BidirectionalIsolateChild {
    fn from(child: Canvas) -> Self {
        BidirectionalIsolateChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        BidirectionalIsolateChild::Canvas(builder.build())
    }
}
impl From<Cite> for BidirectionalIsolateChild {
    fn from(child: Cite) -> Self {
        BidirectionalIsolateChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        BidirectionalIsolateChild::Cite(builder.build())
    }
}
impl From<Code> for BidirectionalIsolateChild {
    fn from(child: Code) -> Self {
        BidirectionalIsolateChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        BidirectionalIsolateChild::Code(builder.build())
    }
}
impl From<Custom> for BidirectionalIsolateChild {
    fn from(child: Custom) -> Self {
        BidirectionalIsolateChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        BidirectionalIsolateChild::Custom(builder.build())
    }
}
impl From<Data> for BidirectionalIsolateChild {
    fn from(child: Data) -> Self {
        BidirectionalIsolateChild::Data(child)
    }
}
impl From<builders::DataBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::DataBuilder) -> Self {
        BidirectionalIsolateChild::Data(builder.build())
    }
}
impl From<DataList> for BidirectionalIsolateChild {
    fn from(child: DataList) -> Self {
        BidirectionalIsolateChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        BidirectionalIsolateChild::DataList(builder.build())
    }
}
impl From<Definition> for BidirectionalIsolateChild {
    fn from(child: Definition) -> Self {
        BidirectionalIsolateChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        BidirectionalIsolateChild::Definition(builder.build())
    }
}
impl From<Deleted> for BidirectionalIsolateChild {
    fn from(child: Deleted) -> Self {
        BidirectionalIsolateChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        BidirectionalIsolateChild::Deleted(builder.build())
    }
}
impl From<Embed> for BidirectionalIsolateChild {
    fn from(child: Embed) -> Self {
        BidirectionalIsolateChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        BidirectionalIsolateChild::Embed(builder.build())
    }
}
impl From<Emphasis> for BidirectionalIsolateChild {
    fn from(child: Emphasis) -> Self {
        BidirectionalIsolateChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        BidirectionalIsolateChild::Emphasis(builder.build())
    }
}
impl From<Image> for BidirectionalIsolateChild {
    fn from(child: Image) -> Self {
        BidirectionalIsolateChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        BidirectionalIsolateChild::Image(builder.build())
    }
}
impl From<InlineFrame> for BidirectionalIsolateChild {
    fn from(child: InlineFrame) -> Self {
        BidirectionalIsolateChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        BidirectionalIsolateChild::InlineFrame(builder.build())
    }
}
impl From<Input> for BidirectionalIsolateChild {
    fn from(child: Input) -> Self {
        BidirectionalIsolateChild::Input(child)
    }
}
impl From<builders::InputBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::InputBuilder) -> Self {
        BidirectionalIsolateChild::Input(builder.build())
    }
}
impl From<Inserted> for BidirectionalIsolateChild {
    fn from(child: Inserted) -> Self {
        BidirectionalIsolateChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        BidirectionalIsolateChild::Inserted(builder.build())
    }
}
impl From<Italic> for BidirectionalIsolateChild {
    fn from(child: Italic) -> Self {
        BidirectionalIsolateChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        BidirectionalIsolateChild::Italic(builder.build())
    }
}
impl From<Keyboard> for BidirectionalIsolateChild {
    fn from(child: Keyboard) -> Self {
        BidirectionalIsolateChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        BidirectionalIsolateChild::Keyboard(builder.build())
    }
}
impl From<Label> for BidirectionalIsolateChild {
    fn from(child: Label) -> Self {
        BidirectionalIsolateChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        BidirectionalIsolateChild::Label(builder.build())
    }
}
impl From<LineBreak> for BidirectionalIsolateChild {
    fn from(child: LineBreak) -> Self {
        BidirectionalIsolateChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        BidirectionalIsolateChild::LineBreak(builder.build())
    }
}
impl From<Link> for BidirectionalIsolateChild {
    fn from(child: Link) -> Self {
        BidirectionalIsolateChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        BidirectionalIsolateChild::Link(builder.build())
    }
}
impl From<Map> for BidirectionalIsolateChild {
    fn from(child: Map) -> Self {
        BidirectionalIsolateChild::Map(child)
    }
}
impl From<builders::MapBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::MapBuilder) -> Self {
        BidirectionalIsolateChild::Map(builder.build())
    }
}
impl From<MapArea> for BidirectionalIsolateChild {
    fn from(child: MapArea) -> Self {
        BidirectionalIsolateChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        BidirectionalIsolateChild::MapArea(builder.build())
    }
}
impl From<Mark> for BidirectionalIsolateChild {
    fn from(child: Mark) -> Self {
        BidirectionalIsolateChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        BidirectionalIsolateChild::Mark(builder.build())
    }
}
impl From<Metadata> for BidirectionalIsolateChild {
    fn from(child: Metadata) -> Self {
        BidirectionalIsolateChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        BidirectionalIsolateChild::Metadata(builder.build())
    }
}
impl From<Meter> for BidirectionalIsolateChild {
    fn from(child: Meter) -> Self {
        BidirectionalIsolateChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        BidirectionalIsolateChild::Meter(builder.build())
    }
}
impl From<NoScript> for BidirectionalIsolateChild {
    fn from(child: NoScript) -> Self {
        BidirectionalIsolateChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        BidirectionalIsolateChild::NoScript(builder.build())
    }
}
impl From<Object> for BidirectionalIsolateChild {
    fn from(child: Object) -> Self {
        BidirectionalIsolateChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        BidirectionalIsolateChild::Object(builder.build())
    }
}
impl From<Output> for BidirectionalIsolateChild {
    fn from(child: Output) -> Self {
        BidirectionalIsolateChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        BidirectionalIsolateChild::Output(builder.build())
    }
}
impl From<Picture> for BidirectionalIsolateChild {
    fn from(child: Picture) -> Self {
        BidirectionalIsolateChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        BidirectionalIsolateChild::Picture(builder.build())
    }
}
impl From<Progress> for BidirectionalIsolateChild {
    fn from(child: Progress) -> Self {
        BidirectionalIsolateChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        BidirectionalIsolateChild::Progress(builder.build())
    }
}
impl From<Quote> for BidirectionalIsolateChild {
    fn from(child: Quote) -> Self {
        BidirectionalIsolateChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        BidirectionalIsolateChild::Quote(builder.build())
    }
}
impl From<Ruby> for BidirectionalIsolateChild {
    fn from(child: Ruby) -> Self {
        BidirectionalIsolateChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        BidirectionalIsolateChild::Ruby(builder.build())
    }
}
impl From<Sample> for BidirectionalIsolateChild {
    fn from(child: Sample) -> Self {
        BidirectionalIsolateChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        BidirectionalIsolateChild::Sample(builder.build())
    }
}
impl From<Script> for BidirectionalIsolateChild {
    fn from(child: Script) -> Self {
        BidirectionalIsolateChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        BidirectionalIsolateChild::Script(builder.build())
    }
}
impl From<Select> for BidirectionalIsolateChild {
    fn from(child: Select) -> Self {
        BidirectionalIsolateChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        BidirectionalIsolateChild::Select(builder.build())
    }
}
impl From<Slot> for BidirectionalIsolateChild {
    fn from(child: Slot) -> Self {
        BidirectionalIsolateChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        BidirectionalIsolateChild::Slot(builder.build())
    }
}
impl From<Small> for BidirectionalIsolateChild {
    fn from(child: Small) -> Self {
        BidirectionalIsolateChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        BidirectionalIsolateChild::Small(builder.build())
    }
}
impl From<Span> for BidirectionalIsolateChild {
    fn from(child: Span) -> Self {
        BidirectionalIsolateChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        BidirectionalIsolateChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for BidirectionalIsolateChild {
    fn from(child: StrikeThrough) -> Self {
        BidirectionalIsolateChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        BidirectionalIsolateChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for BidirectionalIsolateChild {
    fn from(child: Strong) -> Self {
        BidirectionalIsolateChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        BidirectionalIsolateChild::Strong(builder.build())
    }
}
impl From<SubScript> for BidirectionalIsolateChild {
    fn from(child: SubScript) -> Self {
        BidirectionalIsolateChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        BidirectionalIsolateChild::SubScript(builder.build())
    }
}
impl From<SupScript> for BidirectionalIsolateChild {
    fn from(child: SupScript) -> Self {
        BidirectionalIsolateChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        BidirectionalIsolateChild::SupScript(builder.build())
    }
}
impl From<Template> for BidirectionalIsolateChild {
    fn from(child: Template) -> Self {
        BidirectionalIsolateChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        BidirectionalIsolateChild::Template(builder.build())
    }
}
impl From<TextArea> for BidirectionalIsolateChild {
    fn from(child: TextArea) -> Self {
        BidirectionalIsolateChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        BidirectionalIsolateChild::TextArea(builder.build())
    }
}
impl From<Time> for BidirectionalIsolateChild {
    fn from(child: Time) -> Self {
        BidirectionalIsolateChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        BidirectionalIsolateChild::Time(builder.build())
    }
}
impl From<Underline> for BidirectionalIsolateChild {
    fn from(child: Underline) -> Self {
        BidirectionalIsolateChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        BidirectionalIsolateChild::Underline(builder.build())
    }
}
impl From<Variable> for BidirectionalIsolateChild {
    fn from(child: Variable) -> Self {
        BidirectionalIsolateChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        BidirectionalIsolateChild::Variable(builder.build())
    }
}
impl From<Video> for BidirectionalIsolateChild {
    fn from(child: Video) -> Self {
        BidirectionalIsolateChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        BidirectionalIsolateChild::Video(builder.build())
    }
}
impl From<WordBreak> for BidirectionalIsolateChild {
    fn from(child: WordBreak) -> Self {
        BidirectionalIsolateChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for BidirectionalIsolateChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        BidirectionalIsolateChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for BidirectionalIsolateChild {
    fn from(s: &'static str) -> Self {
        BidirectionalIsolateChild::Text(s.into())
    }
}
impl From<String> for BidirectionalIsolateChild {
    fn from(s: String) -> Self {
        BidirectionalIsolateChild::Text(s.into())
    }
}
impl From<CowStr> for BidirectionalIsolateChild {
    fn from(s: CowStr) -> Self {
        BidirectionalIsolateChild::Text(s)
    }
}
impl std::fmt::Debug for BidirectionalIsolateChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BidirectionalIsolateChild::Abbreviation(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalIsolateChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::BidirectionalIsolate(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalIsolateChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalIsolateChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Button(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Code(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Data(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Definition(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalIsolateChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Image(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::InlineFrame(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalIsolateChild::Input(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Label(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Link(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Map(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Object(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Output(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Script(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Select(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Small(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Span(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::StrikeThrough(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BidirectionalIsolateChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Template(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Time(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Video(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            BidirectionalIsolateChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for BidirectionalIsolateChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BidirectionalIsolateChild::Abbreviation(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Audio(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Bold(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Button(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Cite(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Code(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Custom(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Data(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::DataList(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Definition(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Embed(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Emphasis(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Image(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::InlineFrame(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Input(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Inserted(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Italic(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Keyboard(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Label(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::LineBreak(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Link(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Map(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Mark(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Metadata(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Meter(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::NoScript(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Object(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Output(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Picture(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Progress(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Quote(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Sample(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Script(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Select(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Slot(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Small(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Span(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::StrikeThrough(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Strong(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::SubScript(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::SupScript(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Template(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::TextArea(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Time(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::Underline(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Variable(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Video(child) => std::fmt::Display::fmt(child, f),
            BidirectionalIsolateChild::WordBreak(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BidirectionalIsolateChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
