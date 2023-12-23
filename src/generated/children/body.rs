// 🤖 This file is generated!

use crate::*;
/// The `<body>` element's children.
#[derive(Clone)]
pub enum BodyChild {
    Abbreviation(Abbreviation),
    Address(Address),
    Anchor(Anchor),
    Article(Article),
    Aside(Aside),
    Audio(Audio),
    BidirectionalIsolate(BidirectionalIsolate),
    BidirectionalOverride(BidirectionalOverride),
    BlockQuote(BlockQuote),
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
    DescriptionList(DescriptionList),
    Details(Details),
    Dialog(Dialog),
    Division(Division),
    Embed(Embed),
    Emphasis(Emphasis),
    FieldSet(FieldSet),
    Figure(Figure),
    Footer(Footer),
    Form(Form),
    Header(Header),
    Heading1(Heading1),
    Heading2(Heading2),
    Heading3(Heading3),
    Heading4(Heading4),
    Heading5(Heading5),
    Heading6(Heading6),
    HeadingGroup(HeadingGroup),
    HorizontalRule(HorizontalRule),
    Image(Image),
    InlineFrame(InlineFrame),
    Input(Input),
    Inserted(Inserted),
    Italic(Italic),
    Keyboard(Keyboard),
    Label(Label),
    LineBreak(LineBreak),
    Link(Link),
    Main(Main),
    Map(Map),
    MapArea(MapArea),
    Mark(Mark),
    Menu(Menu),
    Metadata(Metadata),
    Meter(Meter),
    Navigation(Navigation),
    NoScript(NoScript),
    Object(Object),
    OrderedList(OrderedList),
    Output(Output),
    Paragraph(Paragraph),
    Picture(Picture),
    Preformatted(Preformatted),
    Progress(Progress),
    Quote(Quote),
    Ruby(Ruby),
    Sample(Sample),
    Script(Script),
    Search(Search),
    Section(Section),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Span(Span),
    StrikeThrough(StrikeThrough),
    Strong(Strong),
    SubScript(SubScript),
    SupScript(SupScript),
    Table(Table),
    Template(Template),
    TextArea(TextArea),
    Time(Time),
    Underline(Underline),
    UnorderedList(UnorderedList),
    Variable(Variable),
    Video(Video),
    WordBreak(WordBreak),
    Text(CowStr),
}
impl From<Abbreviation> for BodyChild {
    fn from(child: Abbreviation) -> Self {
        BodyChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for BodyChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        BodyChild::Abbreviation(builder.build())
    }
}
impl From<Address> for BodyChild {
    fn from(child: Address) -> Self {
        BodyChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for BodyChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        BodyChild::Address(builder.build())
    }
}
impl From<Anchor> for BodyChild {
    fn from(child: Anchor) -> Self {
        BodyChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for BodyChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        BodyChild::Anchor(builder.build())
    }
}
impl From<Article> for BodyChild {
    fn from(child: Article) -> Self {
        BodyChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for BodyChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        BodyChild::Article(builder.build())
    }
}
impl From<Aside> for BodyChild {
    fn from(child: Aside) -> Self {
        BodyChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for BodyChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        BodyChild::Aside(builder.build())
    }
}
impl From<Audio> for BodyChild {
    fn from(child: Audio) -> Self {
        BodyChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for BodyChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        BodyChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for BodyChild {
    fn from(child: BidirectionalIsolate) -> Self {
        BodyChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for BodyChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        BodyChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for BodyChild {
    fn from(child: BidirectionalOverride) -> Self {
        BodyChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for BodyChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        BodyChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for BodyChild {
    fn from(child: BlockQuote) -> Self {
        BodyChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for BodyChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        BodyChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for BodyChild {
    fn from(child: Bold) -> Self {
        BodyChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for BodyChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        BodyChild::Bold(builder.build())
    }
}
impl From<Button> for BodyChild {
    fn from(child: Button) -> Self {
        BodyChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for BodyChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        BodyChild::Button(builder.build())
    }
}
impl From<Canvas> for BodyChild {
    fn from(child: Canvas) -> Self {
        BodyChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for BodyChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        BodyChild::Canvas(builder.build())
    }
}
impl From<Cite> for BodyChild {
    fn from(child: Cite) -> Self {
        BodyChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for BodyChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        BodyChild::Cite(builder.build())
    }
}
impl From<Code> for BodyChild {
    fn from(child: Code) -> Self {
        BodyChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for BodyChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        BodyChild::Code(builder.build())
    }
}
impl From<Custom> for BodyChild {
    fn from(child: Custom) -> Self {
        BodyChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for BodyChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        BodyChild::Custom(builder.build())
    }
}
impl From<Data> for BodyChild {
    fn from(child: Data) -> Self {
        BodyChild::Data(child)
    }
}
impl From<builders::DataBuilder> for BodyChild {
    fn from(builder: builders::DataBuilder) -> Self {
        BodyChild::Data(builder.build())
    }
}
impl From<DataList> for BodyChild {
    fn from(child: DataList) -> Self {
        BodyChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for BodyChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        BodyChild::DataList(builder.build())
    }
}
impl From<Definition> for BodyChild {
    fn from(child: Definition) -> Self {
        BodyChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for BodyChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        BodyChild::Definition(builder.build())
    }
}
impl From<Deleted> for BodyChild {
    fn from(child: Deleted) -> Self {
        BodyChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for BodyChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        BodyChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for BodyChild {
    fn from(child: DescriptionList) -> Self {
        BodyChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for BodyChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        BodyChild::DescriptionList(builder.build())
    }
}
impl From<Details> for BodyChild {
    fn from(child: Details) -> Self {
        BodyChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for BodyChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        BodyChild::Details(builder.build())
    }
}
impl From<Dialog> for BodyChild {
    fn from(child: Dialog) -> Self {
        BodyChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for BodyChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        BodyChild::Dialog(builder.build())
    }
}
impl From<Division> for BodyChild {
    fn from(child: Division) -> Self {
        BodyChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for BodyChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        BodyChild::Division(builder.build())
    }
}
impl From<Embed> for BodyChild {
    fn from(child: Embed) -> Self {
        BodyChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for BodyChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        BodyChild::Embed(builder.build())
    }
}
impl From<Emphasis> for BodyChild {
    fn from(child: Emphasis) -> Self {
        BodyChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for BodyChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        BodyChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for BodyChild {
    fn from(child: FieldSet) -> Self {
        BodyChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for BodyChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        BodyChild::FieldSet(builder.build())
    }
}
impl From<Figure> for BodyChild {
    fn from(child: Figure) -> Self {
        BodyChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for BodyChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        BodyChild::Figure(builder.build())
    }
}
impl From<Footer> for BodyChild {
    fn from(child: Footer) -> Self {
        BodyChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for BodyChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        BodyChild::Footer(builder.build())
    }
}
impl From<Form> for BodyChild {
    fn from(child: Form) -> Self {
        BodyChild::Form(child)
    }
}
impl From<builders::FormBuilder> for BodyChild {
    fn from(builder: builders::FormBuilder) -> Self {
        BodyChild::Form(builder.build())
    }
}
impl From<Header> for BodyChild {
    fn from(child: Header) -> Self {
        BodyChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for BodyChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        BodyChild::Header(builder.build())
    }
}
impl From<Heading1> for BodyChild {
    fn from(child: Heading1) -> Self {
        BodyChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for BodyChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        BodyChild::Heading1(builder.build())
    }
}
impl From<Heading2> for BodyChild {
    fn from(child: Heading2) -> Self {
        BodyChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for BodyChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        BodyChild::Heading2(builder.build())
    }
}
impl From<Heading3> for BodyChild {
    fn from(child: Heading3) -> Self {
        BodyChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for BodyChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        BodyChild::Heading3(builder.build())
    }
}
impl From<Heading4> for BodyChild {
    fn from(child: Heading4) -> Self {
        BodyChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for BodyChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        BodyChild::Heading4(builder.build())
    }
}
impl From<Heading5> for BodyChild {
    fn from(child: Heading5) -> Self {
        BodyChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for BodyChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        BodyChild::Heading5(builder.build())
    }
}
impl From<Heading6> for BodyChild {
    fn from(child: Heading6) -> Self {
        BodyChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for BodyChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        BodyChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for BodyChild {
    fn from(child: HeadingGroup) -> Self {
        BodyChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for BodyChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        BodyChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for BodyChild {
    fn from(child: HorizontalRule) -> Self {
        BodyChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for BodyChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        BodyChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for BodyChild {
    fn from(child: Image) -> Self {
        BodyChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for BodyChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        BodyChild::Image(builder.build())
    }
}
impl From<InlineFrame> for BodyChild {
    fn from(child: InlineFrame) -> Self {
        BodyChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for BodyChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        BodyChild::InlineFrame(builder.build())
    }
}
impl From<Input> for BodyChild {
    fn from(child: Input) -> Self {
        BodyChild::Input(child)
    }
}
impl From<builders::InputBuilder> for BodyChild {
    fn from(builder: builders::InputBuilder) -> Self {
        BodyChild::Input(builder.build())
    }
}
impl From<Inserted> for BodyChild {
    fn from(child: Inserted) -> Self {
        BodyChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for BodyChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        BodyChild::Inserted(builder.build())
    }
}
impl From<Italic> for BodyChild {
    fn from(child: Italic) -> Self {
        BodyChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for BodyChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        BodyChild::Italic(builder.build())
    }
}
impl From<Keyboard> for BodyChild {
    fn from(child: Keyboard) -> Self {
        BodyChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for BodyChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        BodyChild::Keyboard(builder.build())
    }
}
impl From<Label> for BodyChild {
    fn from(child: Label) -> Self {
        BodyChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for BodyChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        BodyChild::Label(builder.build())
    }
}
impl From<LineBreak> for BodyChild {
    fn from(child: LineBreak) -> Self {
        BodyChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for BodyChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        BodyChild::LineBreak(builder.build())
    }
}
impl From<Link> for BodyChild {
    fn from(child: Link) -> Self {
        BodyChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for BodyChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        BodyChild::Link(builder.build())
    }
}
impl From<Main> for BodyChild {
    fn from(child: Main) -> Self {
        BodyChild::Main(child)
    }
}
impl From<builders::MainBuilder> for BodyChild {
    fn from(builder: builders::MainBuilder) -> Self {
        BodyChild::Main(builder.build())
    }
}
impl From<Map> for BodyChild {
    fn from(child: Map) -> Self {
        BodyChild::Map(child)
    }
}
impl From<builders::MapBuilder> for BodyChild {
    fn from(builder: builders::MapBuilder) -> Self {
        BodyChild::Map(builder.build())
    }
}
impl From<MapArea> for BodyChild {
    fn from(child: MapArea) -> Self {
        BodyChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for BodyChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        BodyChild::MapArea(builder.build())
    }
}
impl From<Mark> for BodyChild {
    fn from(child: Mark) -> Self {
        BodyChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for BodyChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        BodyChild::Mark(builder.build())
    }
}
impl From<Menu> for BodyChild {
    fn from(child: Menu) -> Self {
        BodyChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for BodyChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        BodyChild::Menu(builder.build())
    }
}
impl From<Metadata> for BodyChild {
    fn from(child: Metadata) -> Self {
        BodyChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for BodyChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        BodyChild::Metadata(builder.build())
    }
}
impl From<Meter> for BodyChild {
    fn from(child: Meter) -> Self {
        BodyChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for BodyChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        BodyChild::Meter(builder.build())
    }
}
impl From<Navigation> for BodyChild {
    fn from(child: Navigation) -> Self {
        BodyChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for BodyChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        BodyChild::Navigation(builder.build())
    }
}
impl From<NoScript> for BodyChild {
    fn from(child: NoScript) -> Self {
        BodyChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for BodyChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        BodyChild::NoScript(builder.build())
    }
}
impl From<Object> for BodyChild {
    fn from(child: Object) -> Self {
        BodyChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for BodyChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        BodyChild::Object(builder.build())
    }
}
impl From<OrderedList> for BodyChild {
    fn from(child: OrderedList) -> Self {
        BodyChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for BodyChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        BodyChild::OrderedList(builder.build())
    }
}
impl From<Output> for BodyChild {
    fn from(child: Output) -> Self {
        BodyChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for BodyChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        BodyChild::Output(builder.build())
    }
}
impl From<Paragraph> for BodyChild {
    fn from(child: Paragraph) -> Self {
        BodyChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for BodyChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        BodyChild::Paragraph(builder.build())
    }
}
impl From<Picture> for BodyChild {
    fn from(child: Picture) -> Self {
        BodyChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for BodyChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        BodyChild::Picture(builder.build())
    }
}
impl From<Preformatted> for BodyChild {
    fn from(child: Preformatted) -> Self {
        BodyChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for BodyChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        BodyChild::Preformatted(builder.build())
    }
}
impl From<Progress> for BodyChild {
    fn from(child: Progress) -> Self {
        BodyChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for BodyChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        BodyChild::Progress(builder.build())
    }
}
impl From<Quote> for BodyChild {
    fn from(child: Quote) -> Self {
        BodyChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for BodyChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        BodyChild::Quote(builder.build())
    }
}
impl From<Ruby> for BodyChild {
    fn from(child: Ruby) -> Self {
        BodyChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for BodyChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        BodyChild::Ruby(builder.build())
    }
}
impl From<Sample> for BodyChild {
    fn from(child: Sample) -> Self {
        BodyChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for BodyChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        BodyChild::Sample(builder.build())
    }
}
impl From<Script> for BodyChild {
    fn from(child: Script) -> Self {
        BodyChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for BodyChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        BodyChild::Script(builder.build())
    }
}
impl From<Search> for BodyChild {
    fn from(child: Search) -> Self {
        BodyChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for BodyChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        BodyChild::Search(builder.build())
    }
}
impl From<Section> for BodyChild {
    fn from(child: Section) -> Self {
        BodyChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for BodyChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        BodyChild::Section(builder.build())
    }
}
impl From<Select> for BodyChild {
    fn from(child: Select) -> Self {
        BodyChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for BodyChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        BodyChild::Select(builder.build())
    }
}
impl From<Slot> for BodyChild {
    fn from(child: Slot) -> Self {
        BodyChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for BodyChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        BodyChild::Slot(builder.build())
    }
}
impl From<Small> for BodyChild {
    fn from(child: Small) -> Self {
        BodyChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for BodyChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        BodyChild::Small(builder.build())
    }
}
impl From<Span> for BodyChild {
    fn from(child: Span) -> Self {
        BodyChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for BodyChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        BodyChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for BodyChild {
    fn from(child: StrikeThrough) -> Self {
        BodyChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for BodyChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        BodyChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for BodyChild {
    fn from(child: Strong) -> Self {
        BodyChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for BodyChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        BodyChild::Strong(builder.build())
    }
}
impl From<SubScript> for BodyChild {
    fn from(child: SubScript) -> Self {
        BodyChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for BodyChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        BodyChild::SubScript(builder.build())
    }
}
impl From<SupScript> for BodyChild {
    fn from(child: SupScript) -> Self {
        BodyChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for BodyChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        BodyChild::SupScript(builder.build())
    }
}
impl From<Table> for BodyChild {
    fn from(child: Table) -> Self {
        BodyChild::Table(child)
    }
}
impl From<builders::TableBuilder> for BodyChild {
    fn from(builder: builders::TableBuilder) -> Self {
        BodyChild::Table(builder.build())
    }
}
impl From<Template> for BodyChild {
    fn from(child: Template) -> Self {
        BodyChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for BodyChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        BodyChild::Template(builder.build())
    }
}
impl From<TextArea> for BodyChild {
    fn from(child: TextArea) -> Self {
        BodyChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for BodyChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        BodyChild::TextArea(builder.build())
    }
}
impl From<Time> for BodyChild {
    fn from(child: Time) -> Self {
        BodyChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for BodyChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        BodyChild::Time(builder.build())
    }
}
impl From<Underline> for BodyChild {
    fn from(child: Underline) -> Self {
        BodyChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for BodyChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        BodyChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for BodyChild {
    fn from(child: UnorderedList) -> Self {
        BodyChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for BodyChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        BodyChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for BodyChild {
    fn from(child: Variable) -> Self {
        BodyChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for BodyChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        BodyChild::Variable(builder.build())
    }
}
impl From<Video> for BodyChild {
    fn from(child: Video) -> Self {
        BodyChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for BodyChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        BodyChild::Video(builder.build())
    }
}
impl From<WordBreak> for BodyChild {
    fn from(child: WordBreak) -> Self {
        BodyChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for BodyChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        BodyChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for BodyChild {
    fn from(s: &'static str) -> Self {
        BodyChild::Text(s.into())
    }
}
impl From<String> for BodyChild {
    fn from(s: String) -> Self {
        BodyChild::Text(s.into())
    }
}
impl From<CowStr> for BodyChild {
    fn from(s: CowStr) -> Self {
        BodyChild::Text(s)
    }
}
impl std::fmt::Debug for BodyChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Address(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Article(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Button(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Code(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Data(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Details(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Division(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Form(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Header(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Image(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Input(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Label(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Link(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Main(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Map(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Object(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Output(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Script(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Search(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Section(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Select(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Small(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Span(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Table(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Template(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Time(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Video(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            BodyChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for BodyChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Address(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Article(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Aside(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Audio(child) => std::fmt::Display::fmt(child, f),
            BodyChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            BodyChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            BodyChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Bold(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Button(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Cite(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Code(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Custom(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Data(child) => std::fmt::Display::fmt(child, f),
            BodyChild::DataList(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Definition(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            BodyChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Details(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Division(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Embed(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            BodyChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Figure(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Footer(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Form(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Header(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            BodyChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            BodyChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Image(child) => std::fmt::Display::fmt(child, f),
            BodyChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Input(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Italic(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Label(child) => std::fmt::Display::fmt(child, f),
            BodyChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Link(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Main(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Map(child) => std::fmt::Display::fmt(child, f),
            BodyChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Mark(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Menu(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Meter(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            BodyChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Object(child) => std::fmt::Display::fmt(child, f),
            BodyChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Output(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Picture(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Progress(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Quote(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Sample(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Script(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Search(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Section(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Select(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Slot(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Small(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Span(child) => std::fmt::Display::fmt(child, f),
            BodyChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Strong(child) => std::fmt::Display::fmt(child, f),
            BodyChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            BodyChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Table(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Template(child) => std::fmt::Display::fmt(child, f),
            BodyChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Time(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Underline(child) => std::fmt::Display::fmt(child, f),
            BodyChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Variable(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Video(child) => std::fmt::Display::fmt(child, f),
            BodyChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            BodyChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
