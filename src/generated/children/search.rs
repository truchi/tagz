// 🤖 This file is generated!

use crate::*;
/// The `<search>` element's children.
#[derive(Clone)]
pub enum SearchChild {
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
impl From<Abbreviation> for SearchChild {
    fn from(child: Abbreviation) -> Self {
        SearchChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for SearchChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        SearchChild::Abbreviation(builder.build())
    }
}
impl From<Address> for SearchChild {
    fn from(child: Address) -> Self {
        SearchChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for SearchChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        SearchChild::Address(builder.build())
    }
}
impl From<Anchor> for SearchChild {
    fn from(child: Anchor) -> Self {
        SearchChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for SearchChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        SearchChild::Anchor(builder.build())
    }
}
impl From<Article> for SearchChild {
    fn from(child: Article) -> Self {
        SearchChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for SearchChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        SearchChild::Article(builder.build())
    }
}
impl From<Aside> for SearchChild {
    fn from(child: Aside) -> Self {
        SearchChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for SearchChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        SearchChild::Aside(builder.build())
    }
}
impl From<Audio> for SearchChild {
    fn from(child: Audio) -> Self {
        SearchChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for SearchChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        SearchChild::Audio(builder.build())
    }
}
impl From<BidirectionalIsolate> for SearchChild {
    fn from(child: BidirectionalIsolate) -> Self {
        SearchChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for SearchChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        SearchChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for SearchChild {
    fn from(child: BidirectionalOverride) -> Self {
        SearchChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for SearchChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        SearchChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for SearchChild {
    fn from(child: BlockQuote) -> Self {
        SearchChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for SearchChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        SearchChild::BlockQuote(builder.build())
    }
}
impl From<Bold> for SearchChild {
    fn from(child: Bold) -> Self {
        SearchChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for SearchChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        SearchChild::Bold(builder.build())
    }
}
impl From<Button> for SearchChild {
    fn from(child: Button) -> Self {
        SearchChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for SearchChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        SearchChild::Button(builder.build())
    }
}
impl From<Canvas> for SearchChild {
    fn from(child: Canvas) -> Self {
        SearchChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for SearchChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        SearchChild::Canvas(builder.build())
    }
}
impl From<Cite> for SearchChild {
    fn from(child: Cite) -> Self {
        SearchChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for SearchChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        SearchChild::Cite(builder.build())
    }
}
impl From<Code> for SearchChild {
    fn from(child: Code) -> Self {
        SearchChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for SearchChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        SearchChild::Code(builder.build())
    }
}
impl From<Custom> for SearchChild {
    fn from(child: Custom) -> Self {
        SearchChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for SearchChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        SearchChild::Custom(builder.build())
    }
}
impl From<Data> for SearchChild {
    fn from(child: Data) -> Self {
        SearchChild::Data(child)
    }
}
impl From<builders::DataBuilder> for SearchChild {
    fn from(builder: builders::DataBuilder) -> Self {
        SearchChild::Data(builder.build())
    }
}
impl From<DataList> for SearchChild {
    fn from(child: DataList) -> Self {
        SearchChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for SearchChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        SearchChild::DataList(builder.build())
    }
}
impl From<Definition> for SearchChild {
    fn from(child: Definition) -> Self {
        SearchChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for SearchChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        SearchChild::Definition(builder.build())
    }
}
impl From<Deleted> for SearchChild {
    fn from(child: Deleted) -> Self {
        SearchChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for SearchChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        SearchChild::Deleted(builder.build())
    }
}
impl From<DescriptionList> for SearchChild {
    fn from(child: DescriptionList) -> Self {
        SearchChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for SearchChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        SearchChild::DescriptionList(builder.build())
    }
}
impl From<Details> for SearchChild {
    fn from(child: Details) -> Self {
        SearchChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for SearchChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        SearchChild::Details(builder.build())
    }
}
impl From<Dialog> for SearchChild {
    fn from(child: Dialog) -> Self {
        SearchChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for SearchChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        SearchChild::Dialog(builder.build())
    }
}
impl From<Division> for SearchChild {
    fn from(child: Division) -> Self {
        SearchChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for SearchChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        SearchChild::Division(builder.build())
    }
}
impl From<Embed> for SearchChild {
    fn from(child: Embed) -> Self {
        SearchChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for SearchChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        SearchChild::Embed(builder.build())
    }
}
impl From<Emphasis> for SearchChild {
    fn from(child: Emphasis) -> Self {
        SearchChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for SearchChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        SearchChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for SearchChild {
    fn from(child: FieldSet) -> Self {
        SearchChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for SearchChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        SearchChild::FieldSet(builder.build())
    }
}
impl From<Figure> for SearchChild {
    fn from(child: Figure) -> Self {
        SearchChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for SearchChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        SearchChild::Figure(builder.build())
    }
}
impl From<Footer> for SearchChild {
    fn from(child: Footer) -> Self {
        SearchChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for SearchChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        SearchChild::Footer(builder.build())
    }
}
impl From<Form> for SearchChild {
    fn from(child: Form) -> Self {
        SearchChild::Form(child)
    }
}
impl From<builders::FormBuilder> for SearchChild {
    fn from(builder: builders::FormBuilder) -> Self {
        SearchChild::Form(builder.build())
    }
}
impl From<Header> for SearchChild {
    fn from(child: Header) -> Self {
        SearchChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for SearchChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        SearchChild::Header(builder.build())
    }
}
impl From<Heading1> for SearchChild {
    fn from(child: Heading1) -> Self {
        SearchChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for SearchChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        SearchChild::Heading1(builder.build())
    }
}
impl From<Heading2> for SearchChild {
    fn from(child: Heading2) -> Self {
        SearchChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for SearchChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        SearchChild::Heading2(builder.build())
    }
}
impl From<Heading3> for SearchChild {
    fn from(child: Heading3) -> Self {
        SearchChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for SearchChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        SearchChild::Heading3(builder.build())
    }
}
impl From<Heading4> for SearchChild {
    fn from(child: Heading4) -> Self {
        SearchChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for SearchChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        SearchChild::Heading4(builder.build())
    }
}
impl From<Heading5> for SearchChild {
    fn from(child: Heading5) -> Self {
        SearchChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for SearchChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        SearchChild::Heading5(builder.build())
    }
}
impl From<Heading6> for SearchChild {
    fn from(child: Heading6) -> Self {
        SearchChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for SearchChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        SearchChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for SearchChild {
    fn from(child: HeadingGroup) -> Self {
        SearchChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for SearchChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        SearchChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for SearchChild {
    fn from(child: HorizontalRule) -> Self {
        SearchChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for SearchChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        SearchChild::HorizontalRule(builder.build())
    }
}
impl From<Image> for SearchChild {
    fn from(child: Image) -> Self {
        SearchChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for SearchChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        SearchChild::Image(builder.build())
    }
}
impl From<InlineFrame> for SearchChild {
    fn from(child: InlineFrame) -> Self {
        SearchChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for SearchChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        SearchChild::InlineFrame(builder.build())
    }
}
impl From<Input> for SearchChild {
    fn from(child: Input) -> Self {
        SearchChild::Input(child)
    }
}
impl From<builders::InputBuilder> for SearchChild {
    fn from(builder: builders::InputBuilder) -> Self {
        SearchChild::Input(builder.build())
    }
}
impl From<Inserted> for SearchChild {
    fn from(child: Inserted) -> Self {
        SearchChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for SearchChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        SearchChild::Inserted(builder.build())
    }
}
impl From<Italic> for SearchChild {
    fn from(child: Italic) -> Self {
        SearchChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for SearchChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        SearchChild::Italic(builder.build())
    }
}
impl From<Keyboard> for SearchChild {
    fn from(child: Keyboard) -> Self {
        SearchChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for SearchChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        SearchChild::Keyboard(builder.build())
    }
}
impl From<Label> for SearchChild {
    fn from(child: Label) -> Self {
        SearchChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for SearchChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        SearchChild::Label(builder.build())
    }
}
impl From<LineBreak> for SearchChild {
    fn from(child: LineBreak) -> Self {
        SearchChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for SearchChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        SearchChild::LineBreak(builder.build())
    }
}
impl From<Link> for SearchChild {
    fn from(child: Link) -> Self {
        SearchChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for SearchChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        SearchChild::Link(builder.build())
    }
}
impl From<Main> for SearchChild {
    fn from(child: Main) -> Self {
        SearchChild::Main(child)
    }
}
impl From<builders::MainBuilder> for SearchChild {
    fn from(builder: builders::MainBuilder) -> Self {
        SearchChild::Main(builder.build())
    }
}
impl From<Map> for SearchChild {
    fn from(child: Map) -> Self {
        SearchChild::Map(child)
    }
}
impl From<builders::MapBuilder> for SearchChild {
    fn from(builder: builders::MapBuilder) -> Self {
        SearchChild::Map(builder.build())
    }
}
impl From<MapArea> for SearchChild {
    fn from(child: MapArea) -> Self {
        SearchChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for SearchChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        SearchChild::MapArea(builder.build())
    }
}
impl From<Mark> for SearchChild {
    fn from(child: Mark) -> Self {
        SearchChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for SearchChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        SearchChild::Mark(builder.build())
    }
}
impl From<Menu> for SearchChild {
    fn from(child: Menu) -> Self {
        SearchChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for SearchChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        SearchChild::Menu(builder.build())
    }
}
impl From<Metadata> for SearchChild {
    fn from(child: Metadata) -> Self {
        SearchChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for SearchChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        SearchChild::Metadata(builder.build())
    }
}
impl From<Meter> for SearchChild {
    fn from(child: Meter) -> Self {
        SearchChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for SearchChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        SearchChild::Meter(builder.build())
    }
}
impl From<Navigation> for SearchChild {
    fn from(child: Navigation) -> Self {
        SearchChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for SearchChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        SearchChild::Navigation(builder.build())
    }
}
impl From<NoScript> for SearchChild {
    fn from(child: NoScript) -> Self {
        SearchChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for SearchChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        SearchChild::NoScript(builder.build())
    }
}
impl From<Object> for SearchChild {
    fn from(child: Object) -> Self {
        SearchChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for SearchChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        SearchChild::Object(builder.build())
    }
}
impl From<OrderedList> for SearchChild {
    fn from(child: OrderedList) -> Self {
        SearchChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for SearchChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        SearchChild::OrderedList(builder.build())
    }
}
impl From<Output> for SearchChild {
    fn from(child: Output) -> Self {
        SearchChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for SearchChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        SearchChild::Output(builder.build())
    }
}
impl From<Paragraph> for SearchChild {
    fn from(child: Paragraph) -> Self {
        SearchChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for SearchChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        SearchChild::Paragraph(builder.build())
    }
}
impl From<Picture> for SearchChild {
    fn from(child: Picture) -> Self {
        SearchChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for SearchChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        SearchChild::Picture(builder.build())
    }
}
impl From<Preformatted> for SearchChild {
    fn from(child: Preformatted) -> Self {
        SearchChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for SearchChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        SearchChild::Preformatted(builder.build())
    }
}
impl From<Progress> for SearchChild {
    fn from(child: Progress) -> Self {
        SearchChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for SearchChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        SearchChild::Progress(builder.build())
    }
}
impl From<Quote> for SearchChild {
    fn from(child: Quote) -> Self {
        SearchChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for SearchChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        SearchChild::Quote(builder.build())
    }
}
impl From<Ruby> for SearchChild {
    fn from(child: Ruby) -> Self {
        SearchChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for SearchChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        SearchChild::Ruby(builder.build())
    }
}
impl From<Sample> for SearchChild {
    fn from(child: Sample) -> Self {
        SearchChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for SearchChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        SearchChild::Sample(builder.build())
    }
}
impl From<Script> for SearchChild {
    fn from(child: Script) -> Self {
        SearchChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for SearchChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        SearchChild::Script(builder.build())
    }
}
impl From<Search> for SearchChild {
    fn from(child: Search) -> Self {
        SearchChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for SearchChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        SearchChild::Search(builder.build())
    }
}
impl From<Section> for SearchChild {
    fn from(child: Section) -> Self {
        SearchChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for SearchChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        SearchChild::Section(builder.build())
    }
}
impl From<Select> for SearchChild {
    fn from(child: Select) -> Self {
        SearchChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for SearchChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        SearchChild::Select(builder.build())
    }
}
impl From<Slot> for SearchChild {
    fn from(child: Slot) -> Self {
        SearchChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for SearchChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        SearchChild::Slot(builder.build())
    }
}
impl From<Small> for SearchChild {
    fn from(child: Small) -> Self {
        SearchChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for SearchChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        SearchChild::Small(builder.build())
    }
}
impl From<Span> for SearchChild {
    fn from(child: Span) -> Self {
        SearchChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for SearchChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        SearchChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for SearchChild {
    fn from(child: StrikeThrough) -> Self {
        SearchChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for SearchChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        SearchChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for SearchChild {
    fn from(child: Strong) -> Self {
        SearchChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for SearchChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        SearchChild::Strong(builder.build())
    }
}
impl From<SubScript> for SearchChild {
    fn from(child: SubScript) -> Self {
        SearchChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for SearchChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        SearchChild::SubScript(builder.build())
    }
}
impl From<SupScript> for SearchChild {
    fn from(child: SupScript) -> Self {
        SearchChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for SearchChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        SearchChild::SupScript(builder.build())
    }
}
impl From<Table> for SearchChild {
    fn from(child: Table) -> Self {
        SearchChild::Table(child)
    }
}
impl From<builders::TableBuilder> for SearchChild {
    fn from(builder: builders::TableBuilder) -> Self {
        SearchChild::Table(builder.build())
    }
}
impl From<Template> for SearchChild {
    fn from(child: Template) -> Self {
        SearchChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for SearchChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        SearchChild::Template(builder.build())
    }
}
impl From<TextArea> for SearchChild {
    fn from(child: TextArea) -> Self {
        SearchChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for SearchChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        SearchChild::TextArea(builder.build())
    }
}
impl From<Time> for SearchChild {
    fn from(child: Time) -> Self {
        SearchChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for SearchChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        SearchChild::Time(builder.build())
    }
}
impl From<Underline> for SearchChild {
    fn from(child: Underline) -> Self {
        SearchChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for SearchChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        SearchChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for SearchChild {
    fn from(child: UnorderedList) -> Self {
        SearchChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for SearchChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        SearchChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for SearchChild {
    fn from(child: Variable) -> Self {
        SearchChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for SearchChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        SearchChild::Variable(builder.build())
    }
}
impl From<Video> for SearchChild {
    fn from(child: Video) -> Self {
        SearchChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for SearchChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        SearchChild::Video(builder.build())
    }
}
impl From<WordBreak> for SearchChild {
    fn from(child: WordBreak) -> Self {
        SearchChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for SearchChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        SearchChild::WordBreak(builder.build())
    }
}
impl From<&'static str> for SearchChild {
    fn from(s: &'static str) -> Self {
        SearchChild::Text(s.into())
    }
}
impl From<String> for SearchChild {
    fn from(s: String) -> Self {
        SearchChild::Text(s.into())
    }
}
impl From<CowStr> for SearchChild {
    fn from(s: CowStr) -> Self {
        SearchChild::Text(s)
    }
}
impl std::fmt::Debug for SearchChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Address(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Article(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Button(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Code(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Data(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Details(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Division(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Form(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Header(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Image(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Input(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Label(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Link(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Main(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Map(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Object(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Output(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Script(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Search(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Section(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Select(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Small(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Span(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Table(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Template(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Time(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Video(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
            SearchChild::Text(text) => std::fmt::Debug::fmt(text, f),
        }
    }
}
impl std::fmt::Display for SearchChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Address(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Article(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Aside(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Audio(child) => std::fmt::Display::fmt(child, f),
            SearchChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            SearchChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            SearchChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Bold(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Button(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Cite(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Code(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Custom(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Data(child) => std::fmt::Display::fmt(child, f),
            SearchChild::DataList(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Definition(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            SearchChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Details(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Division(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Embed(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            SearchChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Figure(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Footer(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Form(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Header(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            SearchChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            SearchChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Image(child) => std::fmt::Display::fmt(child, f),
            SearchChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Input(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Italic(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Label(child) => std::fmt::Display::fmt(child, f),
            SearchChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Link(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Main(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Map(child) => std::fmt::Display::fmt(child, f),
            SearchChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Mark(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Menu(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Meter(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            SearchChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Object(child) => std::fmt::Display::fmt(child, f),
            SearchChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Output(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Picture(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Progress(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Quote(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Sample(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Script(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Search(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Section(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Select(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Slot(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Small(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Span(child) => std::fmt::Display::fmt(child, f),
            SearchChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Strong(child) => std::fmt::Display::fmt(child, f),
            SearchChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            SearchChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Table(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Template(child) => std::fmt::Display::fmt(child, f),
            SearchChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Time(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Underline(child) => std::fmt::Display::fmt(child, f),
            SearchChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Variable(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Video(child) => std::fmt::Display::fmt(child, f),
            SearchChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
            SearchChild::Text(text) => std::fmt::Display::fmt(text, f),
        }
    }
}
