// 🤖 This file is generated!

use crate::*;
/// The `<blockquote>` element's children.
#[derive(Clone)]
pub enum BlockQuoteChild {
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
impl From<Abbreviation> for BlockQuoteChild {
    fn from(child: Abbreviation) -> Self {
        BlockQuoteChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for BlockQuoteChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        BlockQuoteChild::Abbreviation(builder.build())
    }
}
impl From<Address> for BlockQuoteChild {
    fn from(child: Address) -> Self {
        BlockQuoteChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for BlockQuoteChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        BlockQuoteChild::Address(builder.build())
    }
}
impl From<Anchor> for BlockQuoteChild {
    fn from(child: Anchor) -> Self {
        BlockQuoteChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for BlockQuoteChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        BlockQuoteChild::Anchor(builder.build())
    }
}
impl From<Article> for BlockQuoteChild {
    fn from(child: Article) -> Self {
        BlockQuoteChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for BlockQuoteChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        BlockQuoteChild::Article(builder.build())
    }
}
impl From<Aside> for BlockQuoteChild {
    fn from(child: Aside) -> Self {
        BlockQuoteChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for BlockQuoteChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        BlockQuoteChild::Aside(builder.build())
    }
}
impl From<Audio> for BlockQuoteChild {
    fn from(child: Audio) -> Self {
        BlockQuoteChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for BlockQuoteChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        BlockQuoteChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for BlockQuoteChild {
    fn from(child: BidirectionalIsolate) -> Self {
        BlockQuoteChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for BlockQuoteChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        BlockQuoteChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for BlockQuoteChild {
    fn from(child: BidirectionalOverride) -> Self {
        BlockQuoteChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for BlockQuoteChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        BlockQuoteChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for BlockQuoteChild {
    fn from(child: BlockQuote) -> Self {
        BlockQuoteChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for BlockQuoteChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        BlockQuoteChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for BlockQuoteChild {
    fn from(child: Bold) -> Self {
        BlockQuoteChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for BlockQuoteChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        BlockQuoteChild::Bold(builder.build())
    }
}
impl From<Button> for BlockQuoteChild {
    fn from(child: Button) -> Self {
        BlockQuoteChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for BlockQuoteChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        BlockQuoteChild::Button(builder.build())
    }
}
impl From<Canvas> for BlockQuoteChild {
    fn from(child: Canvas) -> Self {
        BlockQuoteChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for BlockQuoteChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        BlockQuoteChild::Canvas(builder.build())
    }
}
impl From<Cite> for BlockQuoteChild {
    fn from(child: Cite) -> Self {
        BlockQuoteChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for BlockQuoteChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        BlockQuoteChild::Cite(builder.build())
    }
}
impl From<Code> for BlockQuoteChild {
    fn from(child: Code) -> Self {
        BlockQuoteChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for BlockQuoteChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        BlockQuoteChild::Code(builder.build())
    }
}
impl From<Custom> for BlockQuoteChild {
    fn from(child: Custom) -> Self {
        BlockQuoteChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for BlockQuoteChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        BlockQuoteChild::Custom(builder.build())
    }
}
impl From<Data> for BlockQuoteChild {
    fn from(child: Data) -> Self {
        BlockQuoteChild::Data(child)
    }
}
impl From<builders::DataBuilder> for BlockQuoteChild {
    fn from(builder: builders::DataBuilder) -> Self {
        BlockQuoteChild::Data(builder.build())
    }
}
impl From<DataList> for BlockQuoteChild {
    fn from(child: DataList) -> Self {
        BlockQuoteChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for BlockQuoteChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        BlockQuoteChild::DataList(builder.build())
    }
}
impl From<Definition> for BlockQuoteChild {
    fn from(child: Definition) -> Self {
        BlockQuoteChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for BlockQuoteChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        BlockQuoteChild::Definition(builder.build())
    }
}
impl From<Deleted> for BlockQuoteChild {
    fn from(child: Deleted) -> Self {
        BlockQuoteChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for BlockQuoteChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        BlockQuoteChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for BlockQuoteChild {
    fn from(child: DescriptionList) -> Self {
        BlockQuoteChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for BlockQuoteChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        BlockQuoteChild::DescriptionList(builder.build())
    }
}
impl From<Details> for BlockQuoteChild {
    fn from(child: Details) -> Self {
        BlockQuoteChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for BlockQuoteChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        BlockQuoteChild::Details(builder.build())
    }
}
impl From<Dialog> for BlockQuoteChild {
    fn from(child: Dialog) -> Self {
        BlockQuoteChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for BlockQuoteChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        BlockQuoteChild::Dialog(builder.build())
    }
}
impl From<Division> for BlockQuoteChild {
    fn from(child: Division) -> Self {
        BlockQuoteChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for BlockQuoteChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        BlockQuoteChild::Division(builder.build())
    }
}
impl From<Embed> for BlockQuoteChild {
    fn from(child: Embed) -> Self {
        BlockQuoteChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for BlockQuoteChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        BlockQuoteChild::Embed(builder.build())
    }
}
impl From<Emphasis> for BlockQuoteChild {
    fn from(child: Emphasis) -> Self {
        BlockQuoteChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for BlockQuoteChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        BlockQuoteChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for BlockQuoteChild {
    fn from(child: FieldSet) -> Self {
        BlockQuoteChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for BlockQuoteChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        BlockQuoteChild::FieldSet(builder.build())
    }
}
impl From<Figure> for BlockQuoteChild {
    fn from(child: Figure) -> Self {
        BlockQuoteChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for BlockQuoteChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        BlockQuoteChild::Figure(builder.build())
    }
}
impl From<Footer> for BlockQuoteChild {
    fn from(child: Footer) -> Self {
        BlockQuoteChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for BlockQuoteChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        BlockQuoteChild::Footer(builder.build())
    }
}
impl From<Form> for BlockQuoteChild {
    fn from(child: Form) -> Self {
        BlockQuoteChild::Form(child)
    }
}
impl From<builders::FormBuilder> for BlockQuoteChild {
    fn from(builder: builders::FormBuilder) -> Self {
        BlockQuoteChild::Form(builder.build())
    }
}
impl From<Header> for BlockQuoteChild {
    fn from(child: Header) -> Self {
        BlockQuoteChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for BlockQuoteChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        BlockQuoteChild::Header(builder.build())
    }
}
impl From<Heading1> for BlockQuoteChild {
    fn from(child: Heading1) -> Self {
        BlockQuoteChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for BlockQuoteChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        BlockQuoteChild::Heading1(builder.build())
    }
}
impl From<Heading2> for BlockQuoteChild {
    fn from(child: Heading2) -> Self {
        BlockQuoteChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for BlockQuoteChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        BlockQuoteChild::Heading2(builder.build())
    }
}
impl From<Heading3> for BlockQuoteChild {
    fn from(child: Heading3) -> Self {
        BlockQuoteChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for BlockQuoteChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        BlockQuoteChild::Heading3(builder.build())
    }
}
impl From<Heading4> for BlockQuoteChild {
    fn from(child: Heading4) -> Self {
        BlockQuoteChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for BlockQuoteChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        BlockQuoteChild::Heading4(builder.build())
    }
}
impl From<Heading5> for BlockQuoteChild {
    fn from(child: Heading5) -> Self {
        BlockQuoteChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for BlockQuoteChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        BlockQuoteChild::Heading5(builder.build())
    }
}
impl From<Heading6> for BlockQuoteChild {
    fn from(child: Heading6) -> Self {
        BlockQuoteChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for BlockQuoteChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        BlockQuoteChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for BlockQuoteChild {
    fn from(child: HeadingGroup) -> Self {
        BlockQuoteChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for BlockQuoteChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        BlockQuoteChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for BlockQuoteChild {
    fn from(child: HorizontalRule) -> Self {
        BlockQuoteChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for BlockQuoteChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        BlockQuoteChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for BlockQuoteChild {
    fn from(child: Image) -> Self {
        BlockQuoteChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for BlockQuoteChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        BlockQuoteChild::Image(builder.build())
    }
}
impl From<InlineFrame> for BlockQuoteChild {
    fn from(child: InlineFrame) -> Self {
        BlockQuoteChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for BlockQuoteChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        BlockQuoteChild::InlineFrame(builder.build())
    }
}
impl From<Input> for BlockQuoteChild {
    fn from(child: Input) -> Self {
        BlockQuoteChild::Input(child)
    }
}
impl From<builders::InputBuilder> for BlockQuoteChild {
    fn from(builder: builders::InputBuilder) -> Self {
        BlockQuoteChild::Input(builder.build())
    }
}
impl From<Inserted> for BlockQuoteChild {
    fn from(child: Inserted) -> Self {
        BlockQuoteChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for BlockQuoteChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        BlockQuoteChild::Inserted(builder.build())
    }
}
impl From<Italic> for BlockQuoteChild {
    fn from(child: Italic) -> Self {
        BlockQuoteChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for BlockQuoteChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        BlockQuoteChild::Italic(builder.build())
    }
}
impl From<Keyboard> for BlockQuoteChild {
    fn from(child: Keyboard) -> Self {
        BlockQuoteChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for BlockQuoteChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        BlockQuoteChild::Keyboard(builder.build())
    }
}
impl From<Label> for BlockQuoteChild {
    fn from(child: Label) -> Self {
        BlockQuoteChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for BlockQuoteChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        BlockQuoteChild::Label(builder.build())
    }
}
impl From<LineBreak> for BlockQuoteChild {
    fn from(child: LineBreak) -> Self {
        BlockQuoteChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for BlockQuoteChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        BlockQuoteChild::LineBreak(builder.build())
    }
}
impl From<Link> for BlockQuoteChild {
    fn from(child: Link) -> Self {
        BlockQuoteChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for BlockQuoteChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        BlockQuoteChild::Link(builder.build())
    }
}
impl From<Main> for BlockQuoteChild {
    fn from(child: Main) -> Self {
        BlockQuoteChild::Main(child)
    }
}
impl From<builders::MainBuilder> for BlockQuoteChild {
    fn from(builder: builders::MainBuilder) -> Self {
        BlockQuoteChild::Main(builder.build())
    }
}
impl From<Map> for BlockQuoteChild {
    fn from(child: Map) -> Self {
        BlockQuoteChild::Map(child)
    }
}
impl From<builders::MapBuilder> for BlockQuoteChild {
    fn from(builder: builders::MapBuilder) -> Self {
        BlockQuoteChild::Map(builder.build())
    }
}
impl From<MapArea> for BlockQuoteChild {
    fn from(child: MapArea) -> Self {
        BlockQuoteChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for BlockQuoteChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        BlockQuoteChild::MapArea(builder.build())
    }
}
impl From<Mark> for BlockQuoteChild {
    fn from(child: Mark) -> Self {
        BlockQuoteChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for BlockQuoteChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        BlockQuoteChild::Mark(builder.build())
    }
}
impl From<Menu> for BlockQuoteChild {
    fn from(child: Menu) -> Self {
        BlockQuoteChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for BlockQuoteChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        BlockQuoteChild::Menu(builder.build())
    }
}
impl From<Metadata> for BlockQuoteChild {
    fn from(child: Metadata) -> Self {
        BlockQuoteChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for BlockQuoteChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        BlockQuoteChild::Metadata(builder.build())
    }
}
impl From<Meter> for BlockQuoteChild {
    fn from(child: Meter) -> Self {
        BlockQuoteChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for BlockQuoteChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        BlockQuoteChild::Meter(builder.build())
    }
}
impl From<Navigation> for BlockQuoteChild {
    fn from(child: Navigation) -> Self {
        BlockQuoteChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for BlockQuoteChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        BlockQuoteChild::Navigation(builder.build())
    }
}
impl From<NoScript> for BlockQuoteChild {
    fn from(child: NoScript) -> Self {
        BlockQuoteChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for BlockQuoteChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        BlockQuoteChild::NoScript(builder.build())
    }
}
impl From<Object> for BlockQuoteChild {
    fn from(child: Object) -> Self {
        BlockQuoteChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for BlockQuoteChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        BlockQuoteChild::Object(builder.build())
    }
}
impl From<OrderedList> for BlockQuoteChild {
    fn from(child: OrderedList) -> Self {
        BlockQuoteChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for BlockQuoteChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        BlockQuoteChild::OrderedList(builder.build())
    }
}
impl From<Output> for BlockQuoteChild {
    fn from(child: Output) -> Self {
        BlockQuoteChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for BlockQuoteChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        BlockQuoteChild::Output(builder.build())
    }
}
impl From<Paragraph> for BlockQuoteChild {
    fn from(child: Paragraph) -> Self {
        BlockQuoteChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for BlockQuoteChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        BlockQuoteChild::Paragraph(builder.build())
    }
}
impl From<Picture> for BlockQuoteChild {
    fn from(child: Picture) -> Self {
        BlockQuoteChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for BlockQuoteChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        BlockQuoteChild::Picture(builder.build())
    }
}
impl From<Preformatted> for BlockQuoteChild {
    fn from(child: Preformatted) -> Self {
        BlockQuoteChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for BlockQuoteChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        BlockQuoteChild::Preformatted(builder.build())
    }
}
impl From<Progress> for BlockQuoteChild {
    fn from(child: Progress) -> Self {
        BlockQuoteChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for BlockQuoteChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        BlockQuoteChild::Progress(builder.build())
    }
}
impl From<Quote> for BlockQuoteChild {
    fn from(child: Quote) -> Self {
        BlockQuoteChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for BlockQuoteChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        BlockQuoteChild::Quote(builder.build())
    }
}
impl From<Ruby> for BlockQuoteChild {
    fn from(child: Ruby) -> Self {
        BlockQuoteChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for BlockQuoteChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        BlockQuoteChild::Ruby(builder.build())
    }
}
impl From<Sample> for BlockQuoteChild {
    fn from(child: Sample) -> Self {
        BlockQuoteChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for BlockQuoteChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        BlockQuoteChild::Sample(builder.build())
    }
}
impl From<Script> for BlockQuoteChild {
    fn from(child: Script) -> Self {
        BlockQuoteChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for BlockQuoteChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        BlockQuoteChild::Script(builder.build())
    }
}
impl From<Search> for BlockQuoteChild {
    fn from(child: Search) -> Self {
        BlockQuoteChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for BlockQuoteChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        BlockQuoteChild::Search(builder.build())
    }
}
impl From<Section> for BlockQuoteChild {
    fn from(child: Section) -> Self {
        BlockQuoteChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for BlockQuoteChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        BlockQuoteChild::Section(builder.build())
    }
}
impl From<Select> for BlockQuoteChild {
    fn from(child: Select) -> Self {
        BlockQuoteChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for BlockQuoteChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        BlockQuoteChild::Select(builder.build())
    }
}
impl From<Slot> for BlockQuoteChild {
    fn from(child: Slot) -> Self {
        BlockQuoteChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for BlockQuoteChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        BlockQuoteChild::Slot(builder.build())
    }
}
impl From<Small> for BlockQuoteChild {
    fn from(child: Small) -> Self {
        BlockQuoteChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for BlockQuoteChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        BlockQuoteChild::Small(builder.build())
    }
}
impl From<Span> for BlockQuoteChild {
    fn from(child: Span) -> Self {
        BlockQuoteChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for BlockQuoteChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        BlockQuoteChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for BlockQuoteChild {
    fn from(child: StrikeThrough) -> Self {
        BlockQuoteChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for BlockQuoteChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        BlockQuoteChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for BlockQuoteChild {
    fn from(child: Strong) -> Self {
        BlockQuoteChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for BlockQuoteChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        BlockQuoteChild::Strong(builder.build())
    }
}
impl From<SubScript> for BlockQuoteChild {
    fn from(child: SubScript) -> Self {
        BlockQuoteChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for BlockQuoteChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        BlockQuoteChild::SubScript(builder.build())
    }
}
impl From<SupScript> for BlockQuoteChild {
    fn from(child: SupScript) -> Self {
        BlockQuoteChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for BlockQuoteChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        BlockQuoteChild::SupScript(builder.build())
    }
}
impl From<Table> for BlockQuoteChild {
    fn from(child: Table) -> Self {
        BlockQuoteChild::Table(child)
    }
}
impl From<builders::TableBuilder> for BlockQuoteChild {
    fn from(builder: builders::TableBuilder) -> Self {
        BlockQuoteChild::Table(builder.build())
    }
}
impl From<Template> for BlockQuoteChild {
    fn from(child: Template) -> Self {
        BlockQuoteChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for BlockQuoteChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        BlockQuoteChild::Template(builder.build())
    }
}
impl From<TextArea> for BlockQuoteChild {
    fn from(child: TextArea) -> Self {
        BlockQuoteChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for BlockQuoteChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        BlockQuoteChild::TextArea(builder.build())
    }
}
impl From<Time> for BlockQuoteChild {
    fn from(child: Time) -> Self {
        BlockQuoteChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for BlockQuoteChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        BlockQuoteChild::Time(builder.build())
    }
}
impl From<Underline> for BlockQuoteChild {
    fn from(child: Underline) -> Self {
        BlockQuoteChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for BlockQuoteChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        BlockQuoteChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for BlockQuoteChild {
    fn from(child: UnorderedList) -> Self {
        BlockQuoteChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for BlockQuoteChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        BlockQuoteChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for BlockQuoteChild {
    fn from(child: Variable) -> Self {
        BlockQuoteChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for BlockQuoteChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        BlockQuoteChild::Variable(builder.build())
    }
}
impl From<Video> for BlockQuoteChild {
    fn from(child: Video) -> Self {
        BlockQuoteChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for BlockQuoteChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        BlockQuoteChild::Video(builder.build())
    }
}
impl From<WordBreak> for BlockQuoteChild {
    fn from(child: WordBreak) -> Self {
        BlockQuoteChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for BlockQuoteChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        BlockQuoteChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for BlockQuoteChild {
    fn from(s: &'static str) -> Self {
        BlockQuoteChild::Text(s.into())
    }
}
impl From<String> for BlockQuoteChild {
    fn from(s: String) -> Self {
        BlockQuoteChild::Text(s.into())
    }
}
impl From<CowStr> for BlockQuoteChild {
    fn from(s: CowStr) -> Self {
        BlockQuoteChild::Text(s)
    }
}
impl std::fmt::Debug for BlockQuoteChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockQuoteChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Address(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Article(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::BidirectionalIsolate(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BlockQuoteChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            BlockQuoteChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Button(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Code(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Data(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Details(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Division(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Form(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Header(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Image(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Input(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Label(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Link(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Main(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Map(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Object(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Output(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Script(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Search(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Section(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Select(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Small(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Span(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Table(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Template(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Time(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Video(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            BlockQuoteChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for BlockQuoteChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockQuoteChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Address(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Article(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Aside(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Audio(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BlockQuoteChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            BlockQuoteChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Bold(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Button(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Cite(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Code(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Custom(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Data(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::DataList(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Definition(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Details(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Division(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Embed(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Figure(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Footer(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Form(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Header(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Image(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Input(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Italic(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Label(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Link(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Main(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Map(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Mark(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Menu(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Meter(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Object(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Output(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Picture(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Progress(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Quote(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Sample(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Script(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Search(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Section(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Select(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Slot(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Small(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Span(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Strong(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Table(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Template(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Time(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Underline(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Variable(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Video(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            BlockQuoteChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
