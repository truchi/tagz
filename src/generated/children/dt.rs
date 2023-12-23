// 🤖 This file is generated!

use crate::*;
/// The `<dt>` element's children.
#[derive(Clone)]
pub enum DescriptionTermChild {
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
impl From<Abbreviation> for DescriptionTermChild {
    fn from(child: Abbreviation) -> Self {
        DescriptionTermChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for DescriptionTermChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        DescriptionTermChild::Abbreviation(builder.build())
    }
}
impl From<Address> for DescriptionTermChild {
    fn from(child: Address) -> Self {
        DescriptionTermChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for DescriptionTermChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        DescriptionTermChild::Address(builder.build())
    }
}
impl From<Anchor> for DescriptionTermChild {
    fn from(child: Anchor) -> Self {
        DescriptionTermChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for DescriptionTermChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        DescriptionTermChild::Anchor(builder.build())
    }
}
impl From<Article> for DescriptionTermChild {
    fn from(child: Article) -> Self {
        DescriptionTermChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for DescriptionTermChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        DescriptionTermChild::Article(builder.build())
    }
}
impl From<Aside> for DescriptionTermChild {
    fn from(child: Aside) -> Self {
        DescriptionTermChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for DescriptionTermChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        DescriptionTermChild::Aside(builder.build())
    }
}
impl From<Audio> for DescriptionTermChild {
    fn from(child: Audio) -> Self {
        DescriptionTermChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for DescriptionTermChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        DescriptionTermChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for DescriptionTermChild {
    fn from(child: BidirectionalIsolate) -> Self {
        DescriptionTermChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for DescriptionTermChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        DescriptionTermChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for DescriptionTermChild {
    fn from(child: BidirectionalOverride) -> Self {
        DescriptionTermChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for DescriptionTermChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        DescriptionTermChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for DescriptionTermChild {
    fn from(child: BlockQuote) -> Self {
        DescriptionTermChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for DescriptionTermChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        DescriptionTermChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for DescriptionTermChild {
    fn from(child: Bold) -> Self {
        DescriptionTermChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for DescriptionTermChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        DescriptionTermChild::Bold(builder.build())
    }
}
impl From<Button> for DescriptionTermChild {
    fn from(child: Button) -> Self {
        DescriptionTermChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for DescriptionTermChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        DescriptionTermChild::Button(builder.build())
    }
}
impl From<Canvas> for DescriptionTermChild {
    fn from(child: Canvas) -> Self {
        DescriptionTermChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for DescriptionTermChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        DescriptionTermChild::Canvas(builder.build())
    }
}
impl From<Cite> for DescriptionTermChild {
    fn from(child: Cite) -> Self {
        DescriptionTermChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for DescriptionTermChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        DescriptionTermChild::Cite(builder.build())
    }
}
impl From<Code> for DescriptionTermChild {
    fn from(child: Code) -> Self {
        DescriptionTermChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for DescriptionTermChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        DescriptionTermChild::Code(builder.build())
    }
}
impl From<Custom> for DescriptionTermChild {
    fn from(child: Custom) -> Self {
        DescriptionTermChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for DescriptionTermChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        DescriptionTermChild::Custom(builder.build())
    }
}
impl From<Data> for DescriptionTermChild {
    fn from(child: Data) -> Self {
        DescriptionTermChild::Data(child)
    }
}
impl From<builders::DataBuilder> for DescriptionTermChild {
    fn from(builder: builders::DataBuilder) -> Self {
        DescriptionTermChild::Data(builder.build())
    }
}
impl From<DataList> for DescriptionTermChild {
    fn from(child: DataList) -> Self {
        DescriptionTermChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for DescriptionTermChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        DescriptionTermChild::DataList(builder.build())
    }
}
impl From<Definition> for DescriptionTermChild {
    fn from(child: Definition) -> Self {
        DescriptionTermChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for DescriptionTermChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        DescriptionTermChild::Definition(builder.build())
    }
}
impl From<Deleted> for DescriptionTermChild {
    fn from(child: Deleted) -> Self {
        DescriptionTermChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for DescriptionTermChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        DescriptionTermChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for DescriptionTermChild {
    fn from(child: DescriptionList) -> Self {
        DescriptionTermChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for DescriptionTermChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        DescriptionTermChild::DescriptionList(builder.build())
    }
}
impl From<Details> for DescriptionTermChild {
    fn from(child: Details) -> Self {
        DescriptionTermChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for DescriptionTermChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        DescriptionTermChild::Details(builder.build())
    }
}
impl From<Dialog> for DescriptionTermChild {
    fn from(child: Dialog) -> Self {
        DescriptionTermChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for DescriptionTermChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        DescriptionTermChild::Dialog(builder.build())
    }
}
impl From<Division> for DescriptionTermChild {
    fn from(child: Division) -> Self {
        DescriptionTermChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for DescriptionTermChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        DescriptionTermChild::Division(builder.build())
    }
}
impl From<Embed> for DescriptionTermChild {
    fn from(child: Embed) -> Self {
        DescriptionTermChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for DescriptionTermChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        DescriptionTermChild::Embed(builder.build())
    }
}
impl From<Emphasis> for DescriptionTermChild {
    fn from(child: Emphasis) -> Self {
        DescriptionTermChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for DescriptionTermChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        DescriptionTermChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for DescriptionTermChild {
    fn from(child: FieldSet) -> Self {
        DescriptionTermChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for DescriptionTermChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        DescriptionTermChild::FieldSet(builder.build())
    }
}
impl From<Figure> for DescriptionTermChild {
    fn from(child: Figure) -> Self {
        DescriptionTermChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for DescriptionTermChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        DescriptionTermChild::Figure(builder.build())
    }
}
impl From<Footer> for DescriptionTermChild {
    fn from(child: Footer) -> Self {
        DescriptionTermChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for DescriptionTermChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        DescriptionTermChild::Footer(builder.build())
    }
}
impl From<Form> for DescriptionTermChild {
    fn from(child: Form) -> Self {
        DescriptionTermChild::Form(child)
    }
}
impl From<builders::FormBuilder> for DescriptionTermChild {
    fn from(builder: builders::FormBuilder) -> Self {
        DescriptionTermChild::Form(builder.build())
    }
}
impl From<Header> for DescriptionTermChild {
    fn from(child: Header) -> Self {
        DescriptionTermChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for DescriptionTermChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        DescriptionTermChild::Header(builder.build())
    }
}
impl From<Heading1> for DescriptionTermChild {
    fn from(child: Heading1) -> Self {
        DescriptionTermChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for DescriptionTermChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        DescriptionTermChild::Heading1(builder.build())
    }
}
impl From<Heading2> for DescriptionTermChild {
    fn from(child: Heading2) -> Self {
        DescriptionTermChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for DescriptionTermChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        DescriptionTermChild::Heading2(builder.build())
    }
}
impl From<Heading3> for DescriptionTermChild {
    fn from(child: Heading3) -> Self {
        DescriptionTermChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for DescriptionTermChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        DescriptionTermChild::Heading3(builder.build())
    }
}
impl From<Heading4> for DescriptionTermChild {
    fn from(child: Heading4) -> Self {
        DescriptionTermChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for DescriptionTermChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        DescriptionTermChild::Heading4(builder.build())
    }
}
impl From<Heading5> for DescriptionTermChild {
    fn from(child: Heading5) -> Self {
        DescriptionTermChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for DescriptionTermChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        DescriptionTermChild::Heading5(builder.build())
    }
}
impl From<Heading6> for DescriptionTermChild {
    fn from(child: Heading6) -> Self {
        DescriptionTermChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for DescriptionTermChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        DescriptionTermChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for DescriptionTermChild {
    fn from(child: HeadingGroup) -> Self {
        DescriptionTermChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for DescriptionTermChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        DescriptionTermChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for DescriptionTermChild {
    fn from(child: HorizontalRule) -> Self {
        DescriptionTermChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for DescriptionTermChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        DescriptionTermChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for DescriptionTermChild {
    fn from(child: Image) -> Self {
        DescriptionTermChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for DescriptionTermChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        DescriptionTermChild::Image(builder.build())
    }
}
impl From<InlineFrame> for DescriptionTermChild {
    fn from(child: InlineFrame) -> Self {
        DescriptionTermChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for DescriptionTermChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        DescriptionTermChild::InlineFrame(builder.build())
    }
}
impl From<Input> for DescriptionTermChild {
    fn from(child: Input) -> Self {
        DescriptionTermChild::Input(child)
    }
}
impl From<builders::InputBuilder> for DescriptionTermChild {
    fn from(builder: builders::InputBuilder) -> Self {
        DescriptionTermChild::Input(builder.build())
    }
}
impl From<Inserted> for DescriptionTermChild {
    fn from(child: Inserted) -> Self {
        DescriptionTermChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for DescriptionTermChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        DescriptionTermChild::Inserted(builder.build())
    }
}
impl From<Italic> for DescriptionTermChild {
    fn from(child: Italic) -> Self {
        DescriptionTermChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for DescriptionTermChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        DescriptionTermChild::Italic(builder.build())
    }
}
impl From<Keyboard> for DescriptionTermChild {
    fn from(child: Keyboard) -> Self {
        DescriptionTermChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for DescriptionTermChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        DescriptionTermChild::Keyboard(builder.build())
    }
}
impl From<Label> for DescriptionTermChild {
    fn from(child: Label) -> Self {
        DescriptionTermChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for DescriptionTermChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        DescriptionTermChild::Label(builder.build())
    }
}
impl From<LineBreak> for DescriptionTermChild {
    fn from(child: LineBreak) -> Self {
        DescriptionTermChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for DescriptionTermChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        DescriptionTermChild::LineBreak(builder.build())
    }
}
impl From<Link> for DescriptionTermChild {
    fn from(child: Link) -> Self {
        DescriptionTermChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for DescriptionTermChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        DescriptionTermChild::Link(builder.build())
    }
}
impl From<Main> for DescriptionTermChild {
    fn from(child: Main) -> Self {
        DescriptionTermChild::Main(child)
    }
}
impl From<builders::MainBuilder> for DescriptionTermChild {
    fn from(builder: builders::MainBuilder) -> Self {
        DescriptionTermChild::Main(builder.build())
    }
}
impl From<Map> for DescriptionTermChild {
    fn from(child: Map) -> Self {
        DescriptionTermChild::Map(child)
    }
}
impl From<builders::MapBuilder> for DescriptionTermChild {
    fn from(builder: builders::MapBuilder) -> Self {
        DescriptionTermChild::Map(builder.build())
    }
}
impl From<MapArea> for DescriptionTermChild {
    fn from(child: MapArea) -> Self {
        DescriptionTermChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for DescriptionTermChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        DescriptionTermChild::MapArea(builder.build())
    }
}
impl From<Mark> for DescriptionTermChild {
    fn from(child: Mark) -> Self {
        DescriptionTermChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for DescriptionTermChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        DescriptionTermChild::Mark(builder.build())
    }
}
impl From<Menu> for DescriptionTermChild {
    fn from(child: Menu) -> Self {
        DescriptionTermChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for DescriptionTermChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        DescriptionTermChild::Menu(builder.build())
    }
}
impl From<Metadata> for DescriptionTermChild {
    fn from(child: Metadata) -> Self {
        DescriptionTermChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for DescriptionTermChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        DescriptionTermChild::Metadata(builder.build())
    }
}
impl From<Meter> for DescriptionTermChild {
    fn from(child: Meter) -> Self {
        DescriptionTermChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for DescriptionTermChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        DescriptionTermChild::Meter(builder.build())
    }
}
impl From<Navigation> for DescriptionTermChild {
    fn from(child: Navigation) -> Self {
        DescriptionTermChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for DescriptionTermChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        DescriptionTermChild::Navigation(builder.build())
    }
}
impl From<NoScript> for DescriptionTermChild {
    fn from(child: NoScript) -> Self {
        DescriptionTermChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for DescriptionTermChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        DescriptionTermChild::NoScript(builder.build())
    }
}
impl From<Object> for DescriptionTermChild {
    fn from(child: Object) -> Self {
        DescriptionTermChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for DescriptionTermChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        DescriptionTermChild::Object(builder.build())
    }
}
impl From<OrderedList> for DescriptionTermChild {
    fn from(child: OrderedList) -> Self {
        DescriptionTermChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for DescriptionTermChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        DescriptionTermChild::OrderedList(builder.build())
    }
}
impl From<Output> for DescriptionTermChild {
    fn from(child: Output) -> Self {
        DescriptionTermChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for DescriptionTermChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        DescriptionTermChild::Output(builder.build())
    }
}
impl From<Paragraph> for DescriptionTermChild {
    fn from(child: Paragraph) -> Self {
        DescriptionTermChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for DescriptionTermChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        DescriptionTermChild::Paragraph(builder.build())
    }
}
impl From<Picture> for DescriptionTermChild {
    fn from(child: Picture) -> Self {
        DescriptionTermChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for DescriptionTermChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        DescriptionTermChild::Picture(builder.build())
    }
}
impl From<Preformatted> for DescriptionTermChild {
    fn from(child: Preformatted) -> Self {
        DescriptionTermChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for DescriptionTermChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        DescriptionTermChild::Preformatted(builder.build())
    }
}
impl From<Progress> for DescriptionTermChild {
    fn from(child: Progress) -> Self {
        DescriptionTermChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for DescriptionTermChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        DescriptionTermChild::Progress(builder.build())
    }
}
impl From<Quote> for DescriptionTermChild {
    fn from(child: Quote) -> Self {
        DescriptionTermChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for DescriptionTermChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        DescriptionTermChild::Quote(builder.build())
    }
}
impl From<Ruby> for DescriptionTermChild {
    fn from(child: Ruby) -> Self {
        DescriptionTermChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for DescriptionTermChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        DescriptionTermChild::Ruby(builder.build())
    }
}
impl From<Sample> for DescriptionTermChild {
    fn from(child: Sample) -> Self {
        DescriptionTermChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for DescriptionTermChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        DescriptionTermChild::Sample(builder.build())
    }
}
impl From<Script> for DescriptionTermChild {
    fn from(child: Script) -> Self {
        DescriptionTermChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for DescriptionTermChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        DescriptionTermChild::Script(builder.build())
    }
}
impl From<Search> for DescriptionTermChild {
    fn from(child: Search) -> Self {
        DescriptionTermChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for DescriptionTermChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        DescriptionTermChild::Search(builder.build())
    }
}
impl From<Section> for DescriptionTermChild {
    fn from(child: Section) -> Self {
        DescriptionTermChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for DescriptionTermChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        DescriptionTermChild::Section(builder.build())
    }
}
impl From<Select> for DescriptionTermChild {
    fn from(child: Select) -> Self {
        DescriptionTermChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for DescriptionTermChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        DescriptionTermChild::Select(builder.build())
    }
}
impl From<Slot> for DescriptionTermChild {
    fn from(child: Slot) -> Self {
        DescriptionTermChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for DescriptionTermChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        DescriptionTermChild::Slot(builder.build())
    }
}
impl From<Small> for DescriptionTermChild {
    fn from(child: Small) -> Self {
        DescriptionTermChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for DescriptionTermChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        DescriptionTermChild::Small(builder.build())
    }
}
impl From<Span> for DescriptionTermChild {
    fn from(child: Span) -> Self {
        DescriptionTermChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for DescriptionTermChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        DescriptionTermChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for DescriptionTermChild {
    fn from(child: StrikeThrough) -> Self {
        DescriptionTermChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for DescriptionTermChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        DescriptionTermChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for DescriptionTermChild {
    fn from(child: Strong) -> Self {
        DescriptionTermChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for DescriptionTermChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        DescriptionTermChild::Strong(builder.build())
    }
}
impl From<SubScript> for DescriptionTermChild {
    fn from(child: SubScript) -> Self {
        DescriptionTermChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for DescriptionTermChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        DescriptionTermChild::SubScript(builder.build())
    }
}
impl From<SupScript> for DescriptionTermChild {
    fn from(child: SupScript) -> Self {
        DescriptionTermChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for DescriptionTermChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        DescriptionTermChild::SupScript(builder.build())
    }
}
impl From<Table> for DescriptionTermChild {
    fn from(child: Table) -> Self {
        DescriptionTermChild::Table(child)
    }
}
impl From<builders::TableBuilder> for DescriptionTermChild {
    fn from(builder: builders::TableBuilder) -> Self {
        DescriptionTermChild::Table(builder.build())
    }
}
impl From<Template> for DescriptionTermChild {
    fn from(child: Template) -> Self {
        DescriptionTermChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for DescriptionTermChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        DescriptionTermChild::Template(builder.build())
    }
}
impl From<TextArea> for DescriptionTermChild {
    fn from(child: TextArea) -> Self {
        DescriptionTermChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for DescriptionTermChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        DescriptionTermChild::TextArea(builder.build())
    }
}
impl From<Time> for DescriptionTermChild {
    fn from(child: Time) -> Self {
        DescriptionTermChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for DescriptionTermChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        DescriptionTermChild::Time(builder.build())
    }
}
impl From<Underline> for DescriptionTermChild {
    fn from(child: Underline) -> Self {
        DescriptionTermChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for DescriptionTermChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        DescriptionTermChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for DescriptionTermChild {
    fn from(child: UnorderedList) -> Self {
        DescriptionTermChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for DescriptionTermChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        DescriptionTermChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for DescriptionTermChild {
    fn from(child: Variable) -> Self {
        DescriptionTermChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for DescriptionTermChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        DescriptionTermChild::Variable(builder.build())
    }
}
impl From<Video> for DescriptionTermChild {
    fn from(child: Video) -> Self {
        DescriptionTermChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for DescriptionTermChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        DescriptionTermChild::Video(builder.build())
    }
}
impl From<WordBreak> for DescriptionTermChild {
    fn from(child: WordBreak) -> Self {
        DescriptionTermChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for DescriptionTermChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        DescriptionTermChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for DescriptionTermChild {
    fn from(s: &'static str) -> Self {
        DescriptionTermChild::Text(s.into())
    }
}
impl From<String> for DescriptionTermChild {
    fn from(s: String) -> Self {
        DescriptionTermChild::Text(s.into())
    }
}
impl From<CowStr> for DescriptionTermChild {
    fn from(s: CowStr) -> Self {
        DescriptionTermChild::Text(s)
    }
}
impl std::fmt::Debug for DescriptionTermChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DescriptionTermChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Address(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Article(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::BidirectionalIsolate(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DescriptionTermChild::BidirectionalOverride(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DescriptionTermChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Button(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Code(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Data(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::DescriptionList(child) => {
                std::fmt::Debug::fmt(child, f)
            }
            DescriptionTermChild::Details(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Division(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Form(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Header(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Image(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Input(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Label(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Link(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Main(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Map(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Object(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Output(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Script(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Search(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Section(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Select(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Small(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Span(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Table(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Template(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Time(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Video(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            DescriptionTermChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for DescriptionTermChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DescriptionTermChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Address(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Article(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Aside(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Audio(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::BidirectionalIsolate(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionTermChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionTermChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Bold(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Button(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Cite(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Code(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Custom(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Data(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::DataList(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Definition(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::DescriptionList(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionTermChild::Details(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Division(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Embed(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Figure(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Footer(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Form(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Header(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::HorizontalRule(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionTermChild::Image(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Input(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Italic(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Label(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Link(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Main(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Map(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Mark(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Menu(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Meter(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Object(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Output(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Picture(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Progress(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Quote(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Sample(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Script(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Search(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Section(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Select(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Slot(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Small(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Span(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::StrikeThrough(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionTermChild::Strong(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Table(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Template(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Time(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Underline(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::UnorderedList(child) => {
                std::fmt::Display::fmt(child, f)
            }
            DescriptionTermChild::Variable(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Video(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            DescriptionTermChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
