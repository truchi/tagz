// 🤖 This file is generated!

use crate::*;
/// The `<td>` element's children.
#[derive(Clone)]
pub enum TableCellChild {
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
impl From<Abbreviation> for TableCellChild {
    fn from(child: Abbreviation) -> Self {
        TableCellChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for TableCellChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        TableCellChild::Abbreviation(builder.build())
    }
}
impl From<Address> for TableCellChild {
    fn from(child: Address) -> Self {
        TableCellChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for TableCellChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        TableCellChild::Address(builder.build())
    }
}
impl From<Anchor> for TableCellChild {
    fn from(child: Anchor) -> Self {
        TableCellChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for TableCellChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        TableCellChild::Anchor(builder.build())
    }
}
impl From<Article> for TableCellChild {
    fn from(child: Article) -> Self {
        TableCellChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for TableCellChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        TableCellChild::Article(builder.build())
    }
}
impl From<Aside> for TableCellChild {
    fn from(child: Aside) -> Self {
        TableCellChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for TableCellChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        TableCellChild::Aside(builder.build())
    }
}
impl From<Audio> for TableCellChild {
    fn from(child: Audio) -> Self {
        TableCellChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for TableCellChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        TableCellChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for TableCellChild {
    fn from(child: BidirectionalIsolate) -> Self {
        TableCellChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for TableCellChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        TableCellChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for TableCellChild {
    fn from(child: BidirectionalOverride) -> Self {
        TableCellChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for TableCellChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        TableCellChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for TableCellChild {
    fn from(child: BlockQuote) -> Self {
        TableCellChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for TableCellChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        TableCellChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for TableCellChild {
    fn from(child: Bold) -> Self {
        TableCellChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for TableCellChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        TableCellChild::Bold(builder.build())
    }
}
impl From<Button> for TableCellChild {
    fn from(child: Button) -> Self {
        TableCellChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for TableCellChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        TableCellChild::Button(builder.build())
    }
}
impl From<Canvas> for TableCellChild {
    fn from(child: Canvas) -> Self {
        TableCellChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for TableCellChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        TableCellChild::Canvas(builder.build())
    }
}
impl From<Cite> for TableCellChild {
    fn from(child: Cite) -> Self {
        TableCellChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for TableCellChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        TableCellChild::Cite(builder.build())
    }
}
impl From<Code> for TableCellChild {
    fn from(child: Code) -> Self {
        TableCellChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for TableCellChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        TableCellChild::Code(builder.build())
    }
}
impl From<Custom> for TableCellChild {
    fn from(child: Custom) -> Self {
        TableCellChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for TableCellChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        TableCellChild::Custom(builder.build())
    }
}
impl From<Data> for TableCellChild {
    fn from(child: Data) -> Self {
        TableCellChild::Data(child)
    }
}
impl From<builders::DataBuilder> for TableCellChild {
    fn from(builder: builders::DataBuilder) -> Self {
        TableCellChild::Data(builder.build())
    }
}
impl From<DataList> for TableCellChild {
    fn from(child: DataList) -> Self {
        TableCellChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for TableCellChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        TableCellChild::DataList(builder.build())
    }
}
impl From<Definition> for TableCellChild {
    fn from(child: Definition) -> Self {
        TableCellChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for TableCellChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        TableCellChild::Definition(builder.build())
    }
}
impl From<Deleted> for TableCellChild {
    fn from(child: Deleted) -> Self {
        TableCellChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for TableCellChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        TableCellChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for TableCellChild {
    fn from(child: DescriptionList) -> Self {
        TableCellChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for TableCellChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        TableCellChild::DescriptionList(builder.build())
    }
}
impl From<Details> for TableCellChild {
    fn from(child: Details) -> Self {
        TableCellChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for TableCellChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        TableCellChild::Details(builder.build())
    }
}
impl From<Dialog> for TableCellChild {
    fn from(child: Dialog) -> Self {
        TableCellChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for TableCellChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        TableCellChild::Dialog(builder.build())
    }
}
impl From<Division> for TableCellChild {
    fn from(child: Division) -> Self {
        TableCellChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for TableCellChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        TableCellChild::Division(builder.build())
    }
}
impl From<Embed> for TableCellChild {
    fn from(child: Embed) -> Self {
        TableCellChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for TableCellChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        TableCellChild::Embed(builder.build())
    }
}
impl From<Emphasis> for TableCellChild {
    fn from(child: Emphasis) -> Self {
        TableCellChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for TableCellChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        TableCellChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for TableCellChild {
    fn from(child: FieldSet) -> Self {
        TableCellChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for TableCellChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        TableCellChild::FieldSet(builder.build())
    }
}
impl From<Figure> for TableCellChild {
    fn from(child: Figure) -> Self {
        TableCellChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for TableCellChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        TableCellChild::Figure(builder.build())
    }
}
impl From<Footer> for TableCellChild {
    fn from(child: Footer) -> Self {
        TableCellChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for TableCellChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        TableCellChild::Footer(builder.build())
    }
}
impl From<Form> for TableCellChild {
    fn from(child: Form) -> Self {
        TableCellChild::Form(child)
    }
}
impl From<builders::FormBuilder> for TableCellChild {
    fn from(builder: builders::FormBuilder) -> Self {
        TableCellChild::Form(builder.build())
    }
}
impl From<Header> for TableCellChild {
    fn from(child: Header) -> Self {
        TableCellChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for TableCellChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        TableCellChild::Header(builder.build())
    }
}
impl From<Heading1> for TableCellChild {
    fn from(child: Heading1) -> Self {
        TableCellChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for TableCellChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        TableCellChild::Heading1(builder.build())
    }
}
impl From<Heading2> for TableCellChild {
    fn from(child: Heading2) -> Self {
        TableCellChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for TableCellChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        TableCellChild::Heading2(builder.build())
    }
}
impl From<Heading3> for TableCellChild {
    fn from(child: Heading3) -> Self {
        TableCellChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for TableCellChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        TableCellChild::Heading3(builder.build())
    }
}
impl From<Heading4> for TableCellChild {
    fn from(child: Heading4) -> Self {
        TableCellChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for TableCellChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        TableCellChild::Heading4(builder.build())
    }
}
impl From<Heading5> for TableCellChild {
    fn from(child: Heading5) -> Self {
        TableCellChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for TableCellChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        TableCellChild::Heading5(builder.build())
    }
}
impl From<Heading6> for TableCellChild {
    fn from(child: Heading6) -> Self {
        TableCellChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for TableCellChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        TableCellChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for TableCellChild {
    fn from(child: HeadingGroup) -> Self {
        TableCellChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for TableCellChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        TableCellChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for TableCellChild {
    fn from(child: HorizontalRule) -> Self {
        TableCellChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for TableCellChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        TableCellChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for TableCellChild {
    fn from(child: Image) -> Self {
        TableCellChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for TableCellChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        TableCellChild::Image(builder.build())
    }
}
impl From<InlineFrame> for TableCellChild {
    fn from(child: InlineFrame) -> Self {
        TableCellChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for TableCellChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        TableCellChild::InlineFrame(builder.build())
    }
}
impl From<Input> for TableCellChild {
    fn from(child: Input) -> Self {
        TableCellChild::Input(child)
    }
}
impl From<builders::InputBuilder> for TableCellChild {
    fn from(builder: builders::InputBuilder) -> Self {
        TableCellChild::Input(builder.build())
    }
}
impl From<Inserted> for TableCellChild {
    fn from(child: Inserted) -> Self {
        TableCellChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for TableCellChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        TableCellChild::Inserted(builder.build())
    }
}
impl From<Italic> for TableCellChild {
    fn from(child: Italic) -> Self {
        TableCellChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for TableCellChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        TableCellChild::Italic(builder.build())
    }
}
impl From<Keyboard> for TableCellChild {
    fn from(child: Keyboard) -> Self {
        TableCellChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for TableCellChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        TableCellChild::Keyboard(builder.build())
    }
}
impl From<Label> for TableCellChild {
    fn from(child: Label) -> Self {
        TableCellChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for TableCellChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        TableCellChild::Label(builder.build())
    }
}
impl From<LineBreak> for TableCellChild {
    fn from(child: LineBreak) -> Self {
        TableCellChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for TableCellChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        TableCellChild::LineBreak(builder.build())
    }
}
impl From<Link> for TableCellChild {
    fn from(child: Link) -> Self {
        TableCellChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for TableCellChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        TableCellChild::Link(builder.build())
    }
}
impl From<Main> for TableCellChild {
    fn from(child: Main) -> Self {
        TableCellChild::Main(child)
    }
}
impl From<builders::MainBuilder> for TableCellChild {
    fn from(builder: builders::MainBuilder) -> Self {
        TableCellChild::Main(builder.build())
    }
}
impl From<Map> for TableCellChild {
    fn from(child: Map) -> Self {
        TableCellChild::Map(child)
    }
}
impl From<builders::MapBuilder> for TableCellChild {
    fn from(builder: builders::MapBuilder) -> Self {
        TableCellChild::Map(builder.build())
    }
}
impl From<MapArea> for TableCellChild {
    fn from(child: MapArea) -> Self {
        TableCellChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for TableCellChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        TableCellChild::MapArea(builder.build())
    }
}
impl From<Mark> for TableCellChild {
    fn from(child: Mark) -> Self {
        TableCellChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for TableCellChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        TableCellChild::Mark(builder.build())
    }
}
impl From<Menu> for TableCellChild {
    fn from(child: Menu) -> Self {
        TableCellChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for TableCellChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        TableCellChild::Menu(builder.build())
    }
}
impl From<Metadata> for TableCellChild {
    fn from(child: Metadata) -> Self {
        TableCellChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for TableCellChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        TableCellChild::Metadata(builder.build())
    }
}
impl From<Meter> for TableCellChild {
    fn from(child: Meter) -> Self {
        TableCellChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for TableCellChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        TableCellChild::Meter(builder.build())
    }
}
impl From<Navigation> for TableCellChild {
    fn from(child: Navigation) -> Self {
        TableCellChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for TableCellChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        TableCellChild::Navigation(builder.build())
    }
}
impl From<NoScript> for TableCellChild {
    fn from(child: NoScript) -> Self {
        TableCellChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for TableCellChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        TableCellChild::NoScript(builder.build())
    }
}
impl From<Object> for TableCellChild {
    fn from(child: Object) -> Self {
        TableCellChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for TableCellChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        TableCellChild::Object(builder.build())
    }
}
impl From<OrderedList> for TableCellChild {
    fn from(child: OrderedList) -> Self {
        TableCellChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for TableCellChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        TableCellChild::OrderedList(builder.build())
    }
}
impl From<Output> for TableCellChild {
    fn from(child: Output) -> Self {
        TableCellChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for TableCellChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        TableCellChild::Output(builder.build())
    }
}
impl From<Paragraph> for TableCellChild {
    fn from(child: Paragraph) -> Self {
        TableCellChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for TableCellChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        TableCellChild::Paragraph(builder.build())
    }
}
impl From<Picture> for TableCellChild {
    fn from(child: Picture) -> Self {
        TableCellChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for TableCellChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        TableCellChild::Picture(builder.build())
    }
}
impl From<Preformatted> for TableCellChild {
    fn from(child: Preformatted) -> Self {
        TableCellChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for TableCellChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        TableCellChild::Preformatted(builder.build())
    }
}
impl From<Progress> for TableCellChild {
    fn from(child: Progress) -> Self {
        TableCellChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for TableCellChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        TableCellChild::Progress(builder.build())
    }
}
impl From<Quote> for TableCellChild {
    fn from(child: Quote) -> Self {
        TableCellChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for TableCellChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        TableCellChild::Quote(builder.build())
    }
}
impl From<Ruby> for TableCellChild {
    fn from(child: Ruby) -> Self {
        TableCellChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for TableCellChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        TableCellChild::Ruby(builder.build())
    }
}
impl From<Sample> for TableCellChild {
    fn from(child: Sample) -> Self {
        TableCellChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for TableCellChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        TableCellChild::Sample(builder.build())
    }
}
impl From<Script> for TableCellChild {
    fn from(child: Script) -> Self {
        TableCellChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for TableCellChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        TableCellChild::Script(builder.build())
    }
}
impl From<Search> for TableCellChild {
    fn from(child: Search) -> Self {
        TableCellChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for TableCellChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        TableCellChild::Search(builder.build())
    }
}
impl From<Section> for TableCellChild {
    fn from(child: Section) -> Self {
        TableCellChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for TableCellChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        TableCellChild::Section(builder.build())
    }
}
impl From<Select> for TableCellChild {
    fn from(child: Select) -> Self {
        TableCellChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for TableCellChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        TableCellChild::Select(builder.build())
    }
}
impl From<Slot> for TableCellChild {
    fn from(child: Slot) -> Self {
        TableCellChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for TableCellChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        TableCellChild::Slot(builder.build())
    }
}
impl From<Small> for TableCellChild {
    fn from(child: Small) -> Self {
        TableCellChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for TableCellChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        TableCellChild::Small(builder.build())
    }
}
impl From<Span> for TableCellChild {
    fn from(child: Span) -> Self {
        TableCellChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for TableCellChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        TableCellChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for TableCellChild {
    fn from(child: StrikeThrough) -> Self {
        TableCellChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for TableCellChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        TableCellChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for TableCellChild {
    fn from(child: Strong) -> Self {
        TableCellChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for TableCellChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        TableCellChild::Strong(builder.build())
    }
}
impl From<SubScript> for TableCellChild {
    fn from(child: SubScript) -> Self {
        TableCellChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for TableCellChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        TableCellChild::SubScript(builder.build())
    }
}
impl From<SupScript> for TableCellChild {
    fn from(child: SupScript) -> Self {
        TableCellChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for TableCellChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        TableCellChild::SupScript(builder.build())
    }
}
impl From<Table> for TableCellChild {
    fn from(child: Table) -> Self {
        TableCellChild::Table(child)
    }
}
impl From<builders::TableBuilder> for TableCellChild {
    fn from(builder: builders::TableBuilder) -> Self {
        TableCellChild::Table(builder.build())
    }
}
impl From<Template> for TableCellChild {
    fn from(child: Template) -> Self {
        TableCellChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for TableCellChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        TableCellChild::Template(builder.build())
    }
}
impl From<TextArea> for TableCellChild {
    fn from(child: TextArea) -> Self {
        TableCellChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for TableCellChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        TableCellChild::TextArea(builder.build())
    }
}
impl From<Time> for TableCellChild {
    fn from(child: Time) -> Self {
        TableCellChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for TableCellChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        TableCellChild::Time(builder.build())
    }
}
impl From<Underline> for TableCellChild {
    fn from(child: Underline) -> Self {
        TableCellChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for TableCellChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        TableCellChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for TableCellChild {
    fn from(child: UnorderedList) -> Self {
        TableCellChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for TableCellChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        TableCellChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for TableCellChild {
    fn from(child: Variable) -> Self {
        TableCellChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for TableCellChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        TableCellChild::Variable(builder.build())
    }
}
impl From<Video> for TableCellChild {
    fn from(child: Video) -> Self {
        TableCellChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for TableCellChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        TableCellChild::Video(builder.build())
    }
}
impl From<WordBreak> for TableCellChild {
    fn from(child: WordBreak) -> Self {
        TableCellChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for TableCellChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        TableCellChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for TableCellChild {
    fn from(s: &'static str) -> Self {
        TableCellChild::Text(s.into())
    }
}
impl From<String> for TableCellChild {
    fn from(s: String) -> Self {
        TableCellChild::Text(s.into())
    }
}
impl From<CowStr> for TableCellChild {
    fn from(s: CowStr) -> Self {
        TableCellChild::Text(s)
    }
}
impl std::fmt::Debug for TableCellChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableCellChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Address(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Article(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            TableCellChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Button(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Code(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Data(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Details(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Division(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Form(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Header(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Image(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Input(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Label(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Link(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Main(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Map(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Object(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Output(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Script(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Search(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Section(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Select(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Small(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Span(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Table(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Template(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Time(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Video(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            TableCellChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for TableCellChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableCellChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Address(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Article(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Aside(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Audio(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            TableCellChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            TableCellChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Bold(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Button(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Cite(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Code(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Custom(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Data(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::DataList(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Definition(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Details(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Division(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Embed(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Figure(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Footer(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Form(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Header(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Image(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Input(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Italic(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Label(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Link(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Main(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Map(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Mark(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Menu(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Meter(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Object(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Output(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Picture(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Progress(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Quote(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Sample(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Script(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Search(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Section(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Select(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Slot(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Small(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Span(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Strong(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Table(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Template(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Time(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Underline(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Variable(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Video(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            TableCellChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
