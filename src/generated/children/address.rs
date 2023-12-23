// 🤖 This file is generated!

use crate::*;
/// The `<address>` element's children.
#[derive(Clone)]
pub enum AddressChild {
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
impl From<Abbreviation> for AddressChild {
    fn from(child: Abbreviation) -> Self {
        AddressChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for AddressChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        AddressChild::Abbreviation(builder.build())
    }
}
impl From<Address> for AddressChild {
    fn from(child: Address) -> Self {
        AddressChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for AddressChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        AddressChild::Address(builder.build())
    }
}
impl From<Anchor> for AddressChild {
    fn from(child: Anchor) -> Self {
        AddressChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for AddressChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        AddressChild::Anchor(builder.build())
    }
}
impl From<Article> for AddressChild {
    fn from(child: Article) -> Self {
        AddressChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for AddressChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        AddressChild::Article(builder.build())
    }
}
impl From<Aside> for AddressChild {
    fn from(child: Aside) -> Self {
        AddressChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for AddressChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        AddressChild::Aside(builder.build())
    }
}
impl From<Audio> for AddressChild {
    fn from(child: Audio) -> Self {
        AddressChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for AddressChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        AddressChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for AddressChild {
    fn from(child: BidirectionalIsolate) -> Self {
        AddressChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for AddressChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        AddressChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for AddressChild {
    fn from(child: BidirectionalOverride) -> Self {
        AddressChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for AddressChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        AddressChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for AddressChild {
    fn from(child: BlockQuote) -> Self {
        AddressChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for AddressChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        AddressChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for AddressChild {
    fn from(child: Bold) -> Self {
        AddressChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for AddressChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        AddressChild::Bold(builder.build())
    }
}
impl From<Button> for AddressChild {
    fn from(child: Button) -> Self {
        AddressChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for AddressChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        AddressChild::Button(builder.build())
    }
}
impl From<Canvas> for AddressChild {
    fn from(child: Canvas) -> Self {
        AddressChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for AddressChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        AddressChild::Canvas(builder.build())
    }
}
impl From<Cite> for AddressChild {
    fn from(child: Cite) -> Self {
        AddressChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for AddressChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        AddressChild::Cite(builder.build())
    }
}
impl From<Code> for AddressChild {
    fn from(child: Code) -> Self {
        AddressChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for AddressChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        AddressChild::Code(builder.build())
    }
}
impl From<Custom> for AddressChild {
    fn from(child: Custom) -> Self {
        AddressChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for AddressChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        AddressChild::Custom(builder.build())
    }
}
impl From<Data> for AddressChild {
    fn from(child: Data) -> Self {
        AddressChild::Data(child)
    }
}
impl From<builders::DataBuilder> for AddressChild {
    fn from(builder: builders::DataBuilder) -> Self {
        AddressChild::Data(builder.build())
    }
}
impl From<DataList> for AddressChild {
    fn from(child: DataList) -> Self {
        AddressChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for AddressChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        AddressChild::DataList(builder.build())
    }
}
impl From<Definition> for AddressChild {
    fn from(child: Definition) -> Self {
        AddressChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for AddressChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        AddressChild::Definition(builder.build())
    }
}
impl From<Deleted> for AddressChild {
    fn from(child: Deleted) -> Self {
        AddressChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for AddressChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        AddressChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for AddressChild {
    fn from(child: DescriptionList) -> Self {
        AddressChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for AddressChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        AddressChild::DescriptionList(builder.build())
    }
}
impl From<Details> for AddressChild {
    fn from(child: Details) -> Self {
        AddressChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for AddressChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        AddressChild::Details(builder.build())
    }
}
impl From<Dialog> for AddressChild {
    fn from(child: Dialog) -> Self {
        AddressChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for AddressChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        AddressChild::Dialog(builder.build())
    }
}
impl From<Division> for AddressChild {
    fn from(child: Division) -> Self {
        AddressChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for AddressChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        AddressChild::Division(builder.build())
    }
}
impl From<Embed> for AddressChild {
    fn from(child: Embed) -> Self {
        AddressChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for AddressChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        AddressChild::Embed(builder.build())
    }
}
impl From<Emphasis> for AddressChild {
    fn from(child: Emphasis) -> Self {
        AddressChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for AddressChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        AddressChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for AddressChild {
    fn from(child: FieldSet) -> Self {
        AddressChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for AddressChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        AddressChild::FieldSet(builder.build())
    }
}
impl From<Figure> for AddressChild {
    fn from(child: Figure) -> Self {
        AddressChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for AddressChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        AddressChild::Figure(builder.build())
    }
}
impl From<Footer> for AddressChild {
    fn from(child: Footer) -> Self {
        AddressChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for AddressChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        AddressChild::Footer(builder.build())
    }
}
impl From<Form> for AddressChild {
    fn from(child: Form) -> Self {
        AddressChild::Form(child)
    }
}
impl From<builders::FormBuilder> for AddressChild {
    fn from(builder: builders::FormBuilder) -> Self {
        AddressChild::Form(builder.build())
    }
}
impl From<Header> for AddressChild {
    fn from(child: Header) -> Self {
        AddressChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for AddressChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        AddressChild::Header(builder.build())
    }
}
impl From<Heading1> for AddressChild {
    fn from(child: Heading1) -> Self {
        AddressChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for AddressChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        AddressChild::Heading1(builder.build())
    }
}
impl From<Heading2> for AddressChild {
    fn from(child: Heading2) -> Self {
        AddressChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for AddressChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        AddressChild::Heading2(builder.build())
    }
}
impl From<Heading3> for AddressChild {
    fn from(child: Heading3) -> Self {
        AddressChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for AddressChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        AddressChild::Heading3(builder.build())
    }
}
impl From<Heading4> for AddressChild {
    fn from(child: Heading4) -> Self {
        AddressChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for AddressChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        AddressChild::Heading4(builder.build())
    }
}
impl From<Heading5> for AddressChild {
    fn from(child: Heading5) -> Self {
        AddressChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for AddressChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        AddressChild::Heading5(builder.build())
    }
}
impl From<Heading6> for AddressChild {
    fn from(child: Heading6) -> Self {
        AddressChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for AddressChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        AddressChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for AddressChild {
    fn from(child: HeadingGroup) -> Self {
        AddressChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for AddressChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        AddressChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for AddressChild {
    fn from(child: HorizontalRule) -> Self {
        AddressChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for AddressChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        AddressChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for AddressChild {
    fn from(child: Image) -> Self {
        AddressChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for AddressChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        AddressChild::Image(builder.build())
    }
}
impl From<InlineFrame> for AddressChild {
    fn from(child: InlineFrame) -> Self {
        AddressChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for AddressChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        AddressChild::InlineFrame(builder.build())
    }
}
impl From<Input> for AddressChild {
    fn from(child: Input) -> Self {
        AddressChild::Input(child)
    }
}
impl From<builders::InputBuilder> for AddressChild {
    fn from(builder: builders::InputBuilder) -> Self {
        AddressChild::Input(builder.build())
    }
}
impl From<Inserted> for AddressChild {
    fn from(child: Inserted) -> Self {
        AddressChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for AddressChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        AddressChild::Inserted(builder.build())
    }
}
impl From<Italic> for AddressChild {
    fn from(child: Italic) -> Self {
        AddressChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for AddressChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        AddressChild::Italic(builder.build())
    }
}
impl From<Keyboard> for AddressChild {
    fn from(child: Keyboard) -> Self {
        AddressChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for AddressChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        AddressChild::Keyboard(builder.build())
    }
}
impl From<Label> for AddressChild {
    fn from(child: Label) -> Self {
        AddressChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for AddressChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        AddressChild::Label(builder.build())
    }
}
impl From<LineBreak> for AddressChild {
    fn from(child: LineBreak) -> Self {
        AddressChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for AddressChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        AddressChild::LineBreak(builder.build())
    }
}
impl From<Link> for AddressChild {
    fn from(child: Link) -> Self {
        AddressChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for AddressChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        AddressChild::Link(builder.build())
    }
}
impl From<Main> for AddressChild {
    fn from(child: Main) -> Self {
        AddressChild::Main(child)
    }
}
impl From<builders::MainBuilder> for AddressChild {
    fn from(builder: builders::MainBuilder) -> Self {
        AddressChild::Main(builder.build())
    }
}
impl From<Map> for AddressChild {
    fn from(child: Map) -> Self {
        AddressChild::Map(child)
    }
}
impl From<builders::MapBuilder> for AddressChild {
    fn from(builder: builders::MapBuilder) -> Self {
        AddressChild::Map(builder.build())
    }
}
impl From<MapArea> for AddressChild {
    fn from(child: MapArea) -> Self {
        AddressChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for AddressChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        AddressChild::MapArea(builder.build())
    }
}
impl From<Mark> for AddressChild {
    fn from(child: Mark) -> Self {
        AddressChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for AddressChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        AddressChild::Mark(builder.build())
    }
}
impl From<Menu> for AddressChild {
    fn from(child: Menu) -> Self {
        AddressChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for AddressChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        AddressChild::Menu(builder.build())
    }
}
impl From<Metadata> for AddressChild {
    fn from(child: Metadata) -> Self {
        AddressChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for AddressChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        AddressChild::Metadata(builder.build())
    }
}
impl From<Meter> for AddressChild {
    fn from(child: Meter) -> Self {
        AddressChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for AddressChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        AddressChild::Meter(builder.build())
    }
}
impl From<Navigation> for AddressChild {
    fn from(child: Navigation) -> Self {
        AddressChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for AddressChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        AddressChild::Navigation(builder.build())
    }
}
impl From<NoScript> for AddressChild {
    fn from(child: NoScript) -> Self {
        AddressChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for AddressChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        AddressChild::NoScript(builder.build())
    }
}
impl From<Object> for AddressChild {
    fn from(child: Object) -> Self {
        AddressChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for AddressChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        AddressChild::Object(builder.build())
    }
}
impl From<OrderedList> for AddressChild {
    fn from(child: OrderedList) -> Self {
        AddressChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for AddressChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        AddressChild::OrderedList(builder.build())
    }
}
impl From<Output> for AddressChild {
    fn from(child: Output) -> Self {
        AddressChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for AddressChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        AddressChild::Output(builder.build())
    }
}
impl From<Paragraph> for AddressChild {
    fn from(child: Paragraph) -> Self {
        AddressChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for AddressChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        AddressChild::Paragraph(builder.build())
    }
}
impl From<Picture> for AddressChild {
    fn from(child: Picture) -> Self {
        AddressChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for AddressChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        AddressChild::Picture(builder.build())
    }
}
impl From<Preformatted> for AddressChild {
    fn from(child: Preformatted) -> Self {
        AddressChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for AddressChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        AddressChild::Preformatted(builder.build())
    }
}
impl From<Progress> for AddressChild {
    fn from(child: Progress) -> Self {
        AddressChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for AddressChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        AddressChild::Progress(builder.build())
    }
}
impl From<Quote> for AddressChild {
    fn from(child: Quote) -> Self {
        AddressChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for AddressChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        AddressChild::Quote(builder.build())
    }
}
impl From<Ruby> for AddressChild {
    fn from(child: Ruby) -> Self {
        AddressChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for AddressChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        AddressChild::Ruby(builder.build())
    }
}
impl From<Sample> for AddressChild {
    fn from(child: Sample) -> Self {
        AddressChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for AddressChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        AddressChild::Sample(builder.build())
    }
}
impl From<Script> for AddressChild {
    fn from(child: Script) -> Self {
        AddressChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for AddressChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        AddressChild::Script(builder.build())
    }
}
impl From<Search> for AddressChild {
    fn from(child: Search) -> Self {
        AddressChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for AddressChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        AddressChild::Search(builder.build())
    }
}
impl From<Section> for AddressChild {
    fn from(child: Section) -> Self {
        AddressChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for AddressChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        AddressChild::Section(builder.build())
    }
}
impl From<Select> for AddressChild {
    fn from(child: Select) -> Self {
        AddressChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for AddressChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        AddressChild::Select(builder.build())
    }
}
impl From<Slot> for AddressChild {
    fn from(child: Slot) -> Self {
        AddressChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for AddressChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        AddressChild::Slot(builder.build())
    }
}
impl From<Small> for AddressChild {
    fn from(child: Small) -> Self {
        AddressChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for AddressChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        AddressChild::Small(builder.build())
    }
}
impl From<Span> for AddressChild {
    fn from(child: Span) -> Self {
        AddressChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for AddressChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        AddressChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for AddressChild {
    fn from(child: StrikeThrough) -> Self {
        AddressChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for AddressChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        AddressChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for AddressChild {
    fn from(child: Strong) -> Self {
        AddressChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for AddressChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        AddressChild::Strong(builder.build())
    }
}
impl From<SubScript> for AddressChild {
    fn from(child: SubScript) -> Self {
        AddressChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for AddressChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        AddressChild::SubScript(builder.build())
    }
}
impl From<SupScript> for AddressChild {
    fn from(child: SupScript) -> Self {
        AddressChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for AddressChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        AddressChild::SupScript(builder.build())
    }
}
impl From<Table> for AddressChild {
    fn from(child: Table) -> Self {
        AddressChild::Table(child)
    }
}
impl From<builders::TableBuilder> for AddressChild {
    fn from(builder: builders::TableBuilder) -> Self {
        AddressChild::Table(builder.build())
    }
}
impl From<Template> for AddressChild {
    fn from(child: Template) -> Self {
        AddressChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for AddressChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        AddressChild::Template(builder.build())
    }
}
impl From<TextArea> for AddressChild {
    fn from(child: TextArea) -> Self {
        AddressChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for AddressChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        AddressChild::TextArea(builder.build())
    }
}
impl From<Time> for AddressChild {
    fn from(child: Time) -> Self {
        AddressChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for AddressChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        AddressChild::Time(builder.build())
    }
}
impl From<Underline> for AddressChild {
    fn from(child: Underline) -> Self {
        AddressChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for AddressChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        AddressChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for AddressChild {
    fn from(child: UnorderedList) -> Self {
        AddressChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for AddressChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        AddressChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for AddressChild {
    fn from(child: Variable) -> Self {
        AddressChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for AddressChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        AddressChild::Variable(builder.build())
    }
}
impl From<Video> for AddressChild {
    fn from(child: Video) -> Self {
        AddressChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for AddressChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        AddressChild::Video(builder.build())
    }
}
impl From<WordBreak> for AddressChild {
    fn from(child: WordBreak) -> Self {
        AddressChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for AddressChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        AddressChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for AddressChild {
    fn from(s: &'static str) -> Self {
        AddressChild::Text(s.into())
    }
}
impl From<String> for AddressChild {
    fn from(s: String) -> Self {
        AddressChild::Text(s.into())
    }
}
impl From<CowStr> for AddressChild {
    fn from(s: CowStr) -> Self {
        AddressChild::Text(s)
    }
}
impl std::fmt::Debug for AddressChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddressChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Address(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Article(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Button(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Code(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Data(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Details(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Division(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Form(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Header(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Image(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Input(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Label(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Link(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Main(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Map(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Object(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Output(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Script(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Search(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Section(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Select(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Small(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Span(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Table(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Template(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Time(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Video(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            AddressChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for AddressChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddressChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Address(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Article(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Aside(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Audio(child) => std::fmt::Display::fmt(child, f),
            AddressChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            AddressChild::BidirectionalOverride(child) => {
                std::fmt::Display::fmt(child, f)
            }
            AddressChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Bold(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Button(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Cite(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Code(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Custom(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Data(child) => std::fmt::Display::fmt(child, f),
            AddressChild::DataList(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Definition(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            AddressChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Details(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Division(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Embed(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            AddressChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Figure(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Footer(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Form(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Header(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            AddressChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            AddressChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Image(child) => std::fmt::Display::fmt(child, f),
            AddressChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Input(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Italic(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Label(child) => std::fmt::Display::fmt(child, f),
            AddressChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Link(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Main(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Map(child) => std::fmt::Display::fmt(child, f),
            AddressChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Mark(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Menu(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Meter(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            AddressChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Object(child) => std::fmt::Display::fmt(child, f),
            AddressChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Output(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Picture(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Progress(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Quote(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Sample(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Script(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Search(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Section(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Select(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Slot(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Small(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Span(child) => std::fmt::Display::fmt(child, f),
            AddressChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Strong(child) => std::fmt::Display::fmt(child, f),
            AddressChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            AddressChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Table(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Template(child) => std::fmt::Display::fmt(child, f),
            AddressChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Time(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Underline(child) => std::fmt::Display::fmt(child, f),
            AddressChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Variable(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Video(child) => std::fmt::Display::fmt(child, f),
            AddressChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            AddressChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
