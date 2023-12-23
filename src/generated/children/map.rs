// 🤖 This file is generated!

use crate::*;
/// The `<map>` element's children.
#[derive(Clone)]
pub enum MapChild {
    Abbreviation(Abbreviation),
    Address(Address),
    Anchor(Anchor),
    Article(Article),
    Aside(Aside),
    Audio(Audio),
    Base(Base),
    BidirectionalIsolate(BidirectionalIsolate),
    BidirectionalOverride(BidirectionalOverride),
    BlockQuote(BlockQuote),
    Body(Body),
    Bold(Bold),
    Button(Button),
    Canvas(Canvas),
    Caption(Caption),
    Cite(Cite),
    Code(Code),
    Custom(Custom),
    Data(Data),
    DataList(DataList),
    Definition(Definition),
    Deleted(Deleted),
    DescriptionDetails(DescriptionDetails),
    DescriptionList(DescriptionList),
    DescriptionTerm(DescriptionTerm),
    Details(Details),
    Dialog(Dialog),
    Division(Division),
    Embed(Embed),
    Emphasis(Emphasis),
    FieldSet(FieldSet),
    FieldSetLegend(FieldSetLegend),
    Figure(Figure),
    FigureCaption(FigureCaption),
    Footer(Footer),
    Form(Form),
    Head(Head),
    Header(Header),
    Heading1(Heading1),
    Heading2(Heading2),
    Heading3(Heading3),
    Heading4(Heading4),
    Heading5(Heading5),
    Heading6(Heading6),
    HeadingGroup(HeadingGroup),
    HorizontalRule(HorizontalRule),
    Html(Html),
    Image(Image),
    InlineFrame(InlineFrame),
    Input(Input),
    Inserted(Inserted),
    Italic(Italic),
    Keyboard(Keyboard),
    Label(Label),
    LineBreak(LineBreak),
    Link(Link),
    ListItem(ListItem),
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
    Option(Option),
    OptionGroup(OptionGroup),
    OrderedList(OrderedList),
    Output(Output),
    Paragraph(Paragraph),
    Picture(Picture),
    Preformatted(Preformatted),
    Progress(Progress),
    Quote(Quote),
    Ruby(Ruby),
    RubyParenthesis(RubyParenthesis),
    RubyText(RubyText),
    Sample(Sample),
    Script(Script),
    Search(Search),
    Section(Section),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Source(Source),
    Span(Span),
    StrikeThrough(StrikeThrough),
    Strong(Strong),
    Style(Style),
    SubScript(SubScript),
    Summary(Summary),
    SupScript(SupScript),
    Table(Table),
    TableBody(TableBody),
    TableCell(TableCell),
    TableColumn(TableColumn),
    TableColumnGroup(TableColumnGroup),
    TableFoot(TableFoot),
    TableHead(TableHead),
    TableHeader(TableHeader),
    TableRow(TableRow),
    Template(Template),
    TextArea(TextArea),
    Time(Time),
    Title(Title),
    Track(Track),
    Underline(Underline),
    UnorderedList(UnorderedList),
    Variable(Variable),
    Video(Video),
    WordBreak(WordBreak),
}
impl From<Abbreviation> for MapChild {
    fn from(child: Abbreviation) -> Self {
        MapChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for MapChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        MapChild::Abbreviation(builder.build())
    }
}
impl From<Address> for MapChild {
    fn from(child: Address) -> Self {
        MapChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for MapChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        MapChild::Address(builder.build())
    }
}
impl From<Anchor> for MapChild {
    fn from(child: Anchor) -> Self {
        MapChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for MapChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        MapChild::Anchor(builder.build())
    }
}
impl From<Article> for MapChild {
    fn from(child: Article) -> Self {
        MapChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for MapChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        MapChild::Article(builder.build())
    }
}
impl From<Aside> for MapChild {
    fn from(child: Aside) -> Self {
        MapChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for MapChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        MapChild::Aside(builder.build())
    }
}
impl From<Audio> for MapChild {
    fn from(child: Audio) -> Self {
        MapChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for MapChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        MapChild::Audio(builder.build())
    }
}
impl From<Base> for MapChild {
    fn from(child: Base) -> Self {
        MapChild::Base(child)
    }
}
impl From<builders::BaseBuilder> for MapChild {
    fn from(builder: builders::BaseBuilder) -> Self {
        MapChild::Base(builder.build())
    }
}
impl From<BidirectionalIsolate> for MapChild {
    fn from(child: BidirectionalIsolate) -> Self {
        MapChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for MapChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        MapChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for MapChild {
    fn from(child: BidirectionalOverride) -> Self {
        MapChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for MapChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        MapChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for MapChild {
    fn from(child: BlockQuote) -> Self {
        MapChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for MapChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        MapChild::BlockQuote(builder.build())
    }
}
impl From<Body> for MapChild {
    fn from(child: Body) -> Self {
        MapChild::Body(child)
    }
}
impl From<builders::BodyBuilder> for MapChild {
    fn from(builder: builders::BodyBuilder) -> Self {
        MapChild::Body(builder.build())
    }
}
impl From<Bold> for MapChild {
    fn from(child: Bold) -> Self {
        MapChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for MapChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        MapChild::Bold(builder.build())
    }
}
impl From<Button> for MapChild {
    fn from(child: Button) -> Self {
        MapChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for MapChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        MapChild::Button(builder.build())
    }
}
impl From<Canvas> for MapChild {
    fn from(child: Canvas) -> Self {
        MapChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for MapChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        MapChild::Canvas(builder.build())
    }
}
impl From<Caption> for MapChild {
    fn from(child: Caption) -> Self {
        MapChild::Caption(child)
    }
}
impl From<builders::CaptionBuilder> for MapChild {
    fn from(builder: builders::CaptionBuilder) -> Self {
        MapChild::Caption(builder.build())
    }
}
impl From<Cite> for MapChild {
    fn from(child: Cite) -> Self {
        MapChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for MapChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        MapChild::Cite(builder.build())
    }
}
impl From<Code> for MapChild {
    fn from(child: Code) -> Self {
        MapChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for MapChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        MapChild::Code(builder.build())
    }
}
impl From<Custom> for MapChild {
    fn from(child: Custom) -> Self {
        MapChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for MapChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        MapChild::Custom(builder.build())
    }
}
impl From<Data> for MapChild {
    fn from(child: Data) -> Self {
        MapChild::Data(child)
    }
}
impl From<builders::DataBuilder> for MapChild {
    fn from(builder: builders::DataBuilder) -> Self {
        MapChild::Data(builder.build())
    }
}
impl From<DataList> for MapChild {
    fn from(child: DataList) -> Self {
        MapChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for MapChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        MapChild::DataList(builder.build())
    }
}
impl From<Definition> for MapChild {
    fn from(child: Definition) -> Self {
        MapChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for MapChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        MapChild::Definition(builder.build())
    }
}
impl From<Deleted> for MapChild {
    fn from(child: Deleted) -> Self {
        MapChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for MapChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        MapChild::Deleted(builder.build())
    }
}
impl From<DescriptionDetails> for MapChild {
    fn from(child: DescriptionDetails) -> Self {
        MapChild::DescriptionDetails(child)
    }
}
impl From<builders::DescriptionDetailsBuilder> for MapChild {
    fn from(builder: builders::DescriptionDetailsBuilder) -> Self {
        MapChild::DescriptionDetails(builder.build())
    }
}
impl From<DescriptionList> for MapChild {
    fn from(child: DescriptionList) -> Self {
        MapChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for MapChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        MapChild::DescriptionList(builder.build())
    }
}
impl From<DescriptionTerm> for MapChild {
    fn from(child: DescriptionTerm) -> Self {
        MapChild::DescriptionTerm(child)
    }
}
impl From<builders::DescriptionTermBuilder> for MapChild {
    fn from(builder: builders::DescriptionTermBuilder) -> Self {
        MapChild::DescriptionTerm(builder.build())
    }
}
impl From<Details> for MapChild {
    fn from(child: Details) -> Self {
        MapChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for MapChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        MapChild::Details(builder.build())
    }
}
impl From<Dialog> for MapChild {
    fn from(child: Dialog) -> Self {
        MapChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for MapChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        MapChild::Dialog(builder.build())
    }
}
impl From<Division> for MapChild {
    fn from(child: Division) -> Self {
        MapChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for MapChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        MapChild::Division(builder.build())
    }
}
impl From<Embed> for MapChild {
    fn from(child: Embed) -> Self {
        MapChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for MapChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        MapChild::Embed(builder.build())
    }
}
impl From<Emphasis> for MapChild {
    fn from(child: Emphasis) -> Self {
        MapChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for MapChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        MapChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for MapChild {
    fn from(child: FieldSet) -> Self {
        MapChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for MapChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        MapChild::FieldSet(builder.build())
    }
}
impl From<FieldSetLegend> for MapChild {
    fn from(child: FieldSetLegend) -> Self {
        MapChild::FieldSetLegend(child)
    }
}
impl From<builders::FieldSetLegendBuilder> for MapChild {
    fn from(builder: builders::FieldSetLegendBuilder) -> Self {
        MapChild::FieldSetLegend(builder.build())
    }
}
impl From<Figure> for MapChild {
    fn from(child: Figure) -> Self {
        MapChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for MapChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        MapChild::Figure(builder.build())
    }
}
impl From<FigureCaption> for MapChild {
    fn from(child: FigureCaption) -> Self {
        MapChild::FigureCaption(child)
    }
}
impl From<builders::FigureCaptionBuilder> for MapChild {
    fn from(builder: builders::FigureCaptionBuilder) -> Self {
        MapChild::FigureCaption(builder.build())
    }
}
impl From<Footer> for MapChild {
    fn from(child: Footer) -> Self {
        MapChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for MapChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        MapChild::Footer(builder.build())
    }
}
impl From<Form> for MapChild {
    fn from(child: Form) -> Self {
        MapChild::Form(child)
    }
}
impl From<builders::FormBuilder> for MapChild {
    fn from(builder: builders::FormBuilder) -> Self {
        MapChild::Form(builder.build())
    }
}
impl From<Head> for MapChild {
    fn from(child: Head) -> Self {
        MapChild::Head(child)
    }
}
impl From<builders::HeadBuilder> for MapChild {
    fn from(builder: builders::HeadBuilder) -> Self {
        MapChild::Head(builder.build())
    }
}
impl From<Header> for MapChild {
    fn from(child: Header) -> Self {
        MapChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for MapChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        MapChild::Header(builder.build())
    }
}
impl From<Heading1> for MapChild {
    fn from(child: Heading1) -> Self {
        MapChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for MapChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        MapChild::Heading1(builder.build())
    }
}
impl From<Heading2> for MapChild {
    fn from(child: Heading2) -> Self {
        MapChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for MapChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        MapChild::Heading2(builder.build())
    }
}
impl From<Heading3> for MapChild {
    fn from(child: Heading3) -> Self {
        MapChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for MapChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        MapChild::Heading3(builder.build())
    }
}
impl From<Heading4> for MapChild {
    fn from(child: Heading4) -> Self {
        MapChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for MapChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        MapChild::Heading4(builder.build())
    }
}
impl From<Heading5> for MapChild {
    fn from(child: Heading5) -> Self {
        MapChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for MapChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        MapChild::Heading5(builder.build())
    }
}
impl From<Heading6> for MapChild {
    fn from(child: Heading6) -> Self {
        MapChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for MapChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        MapChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for MapChild {
    fn from(child: HeadingGroup) -> Self {
        MapChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for MapChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        MapChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for MapChild {
    fn from(child: HorizontalRule) -> Self {
        MapChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for MapChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        MapChild::HorizontalRule(builder.build())
    }
}
impl From<Html> for MapChild {
    fn from(child: Html) -> Self {
        MapChild::Html(child)
    }
}
impl From<builders::HtmlBuilder> for MapChild {
    fn from(builder: builders::HtmlBuilder) -> Self {
        MapChild::Html(builder.build())
    }
}
impl From<Image> for MapChild {
    fn from(child: Image) -> Self {
        MapChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for MapChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        MapChild::Image(builder.build())
    }
}
impl From<InlineFrame> for MapChild {
    fn from(child: InlineFrame) -> Self {
        MapChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for MapChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        MapChild::InlineFrame(builder.build())
    }
}
impl From<Input> for MapChild {
    fn from(child: Input) -> Self {
        MapChild::Input(child)
    }
}
impl From<builders::InputBuilder> for MapChild {
    fn from(builder: builders::InputBuilder) -> Self {
        MapChild::Input(builder.build())
    }
}
impl From<Inserted> for MapChild {
    fn from(child: Inserted) -> Self {
        MapChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for MapChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        MapChild::Inserted(builder.build())
    }
}
impl From<Italic> for MapChild {
    fn from(child: Italic) -> Self {
        MapChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for MapChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        MapChild::Italic(builder.build())
    }
}
impl From<Keyboard> for MapChild {
    fn from(child: Keyboard) -> Self {
        MapChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for MapChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        MapChild::Keyboard(builder.build())
    }
}
impl From<Label> for MapChild {
    fn from(child: Label) -> Self {
        MapChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for MapChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        MapChild::Label(builder.build())
    }
}
impl From<LineBreak> for MapChild {
    fn from(child: LineBreak) -> Self {
        MapChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for MapChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        MapChild::LineBreak(builder.build())
    }
}
impl From<Link> for MapChild {
    fn from(child: Link) -> Self {
        MapChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for MapChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        MapChild::Link(builder.build())
    }
}
impl From<ListItem> for MapChild {
    fn from(child: ListItem) -> Self {
        MapChild::ListItem(child)
    }
}
impl From<builders::ListItemBuilder> for MapChild {
    fn from(builder: builders::ListItemBuilder) -> Self {
        MapChild::ListItem(builder.build())
    }
}
impl From<Main> for MapChild {
    fn from(child: Main) -> Self {
        MapChild::Main(child)
    }
}
impl From<builders::MainBuilder> for MapChild {
    fn from(builder: builders::MainBuilder) -> Self {
        MapChild::Main(builder.build())
    }
}
impl From<Map> for MapChild {
    fn from(child: Map) -> Self {
        MapChild::Map(child)
    }
}
impl From<builders::MapBuilder> for MapChild {
    fn from(builder: builders::MapBuilder) -> Self {
        MapChild::Map(builder.build())
    }
}
impl From<MapArea> for MapChild {
    fn from(child: MapArea) -> Self {
        MapChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for MapChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        MapChild::MapArea(builder.build())
    }
}
impl From<Mark> for MapChild {
    fn from(child: Mark) -> Self {
        MapChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for MapChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        MapChild::Mark(builder.build())
    }
}
impl From<Menu> for MapChild {
    fn from(child: Menu) -> Self {
        MapChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for MapChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        MapChild::Menu(builder.build())
    }
}
impl From<Metadata> for MapChild {
    fn from(child: Metadata) -> Self {
        MapChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for MapChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        MapChild::Metadata(builder.build())
    }
}
impl From<Meter> for MapChild {
    fn from(child: Meter) -> Self {
        MapChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for MapChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        MapChild::Meter(builder.build())
    }
}
impl From<Navigation> for MapChild {
    fn from(child: Navigation) -> Self {
        MapChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for MapChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        MapChild::Navigation(builder.build())
    }
}
impl From<NoScript> for MapChild {
    fn from(child: NoScript) -> Self {
        MapChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for MapChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        MapChild::NoScript(builder.build())
    }
}
impl From<Object> for MapChild {
    fn from(child: Object) -> Self {
        MapChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for MapChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        MapChild::Object(builder.build())
    }
}
impl From<Option> for MapChild {
    fn from(child: Option) -> Self {
        MapChild::Option(child)
    }
}
impl From<builders::OptionBuilder> for MapChild {
    fn from(builder: builders::OptionBuilder) -> Self {
        MapChild::Option(builder.build())
    }
}
impl From<OptionGroup> for MapChild {
    fn from(child: OptionGroup) -> Self {
        MapChild::OptionGroup(child)
    }
}
impl From<builders::OptionGroupBuilder> for MapChild {
    fn from(builder: builders::OptionGroupBuilder) -> Self {
        MapChild::OptionGroup(builder.build())
    }
}
impl From<OrderedList> for MapChild {
    fn from(child: OrderedList) -> Self {
        MapChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for MapChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        MapChild::OrderedList(builder.build())
    }
}
impl From<Output> for MapChild {
    fn from(child: Output) -> Self {
        MapChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for MapChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        MapChild::Output(builder.build())
    }
}
impl From<Paragraph> for MapChild {
    fn from(child: Paragraph) -> Self {
        MapChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for MapChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        MapChild::Paragraph(builder.build())
    }
}
impl From<Picture> for MapChild {
    fn from(child: Picture) -> Self {
        MapChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for MapChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        MapChild::Picture(builder.build())
    }
}
impl From<Preformatted> for MapChild {
    fn from(child: Preformatted) -> Self {
        MapChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for MapChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        MapChild::Preformatted(builder.build())
    }
}
impl From<Progress> for MapChild {
    fn from(child: Progress) -> Self {
        MapChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for MapChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        MapChild::Progress(builder.build())
    }
}
impl From<Quote> for MapChild {
    fn from(child: Quote) -> Self {
        MapChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for MapChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        MapChild::Quote(builder.build())
    }
}
impl From<Ruby> for MapChild {
    fn from(child: Ruby) -> Self {
        MapChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for MapChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        MapChild::Ruby(builder.build())
    }
}
impl From<RubyParenthesis> for MapChild {
    fn from(child: RubyParenthesis) -> Self {
        MapChild::RubyParenthesis(child)
    }
}
impl From<builders::RubyParenthesisBuilder> for MapChild {
    fn from(builder: builders::RubyParenthesisBuilder) -> Self {
        MapChild::RubyParenthesis(builder.build())
    }
}
impl From<RubyText> for MapChild {
    fn from(child: RubyText) -> Self {
        MapChild::RubyText(child)
    }
}
impl From<builders::RubyTextBuilder> for MapChild {
    fn from(builder: builders::RubyTextBuilder) -> Self {
        MapChild::RubyText(builder.build())
    }
}
impl From<Sample> for MapChild {
    fn from(child: Sample) -> Self {
        MapChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for MapChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        MapChild::Sample(builder.build())
    }
}
impl From<Script> for MapChild {
    fn from(child: Script) -> Self {
        MapChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for MapChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        MapChild::Script(builder.build())
    }
}
impl From<Search> for MapChild {
    fn from(child: Search) -> Self {
        MapChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for MapChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        MapChild::Search(builder.build())
    }
}
impl From<Section> for MapChild {
    fn from(child: Section) -> Self {
        MapChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for MapChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        MapChild::Section(builder.build())
    }
}
impl From<Select> for MapChild {
    fn from(child: Select) -> Self {
        MapChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for MapChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        MapChild::Select(builder.build())
    }
}
impl From<Slot> for MapChild {
    fn from(child: Slot) -> Self {
        MapChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for MapChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        MapChild::Slot(builder.build())
    }
}
impl From<Small> for MapChild {
    fn from(child: Small) -> Self {
        MapChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for MapChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        MapChild::Small(builder.build())
    }
}
impl From<Source> for MapChild {
    fn from(child: Source) -> Self {
        MapChild::Source(child)
    }
}
impl From<builders::SourceBuilder> for MapChild {
    fn from(builder: builders::SourceBuilder) -> Self {
        MapChild::Source(builder.build())
    }
}
impl From<Span> for MapChild {
    fn from(child: Span) -> Self {
        MapChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for MapChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        MapChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for MapChild {
    fn from(child: StrikeThrough) -> Self {
        MapChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for MapChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        MapChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for MapChild {
    fn from(child: Strong) -> Self {
        MapChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for MapChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        MapChild::Strong(builder.build())
    }
}
impl From<Style> for MapChild {
    fn from(child: Style) -> Self {
        MapChild::Style(child)
    }
}
impl From<builders::StyleBuilder> for MapChild {
    fn from(builder: builders::StyleBuilder) -> Self {
        MapChild::Style(builder.build())
    }
}
impl From<SubScript> for MapChild {
    fn from(child: SubScript) -> Self {
        MapChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for MapChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        MapChild::SubScript(builder.build())
    }
}
impl From<Summary> for MapChild {
    fn from(child: Summary) -> Self {
        MapChild::Summary(child)
    }
}
impl From<builders::SummaryBuilder> for MapChild {
    fn from(builder: builders::SummaryBuilder) -> Self {
        MapChild::Summary(builder.build())
    }
}
impl From<SupScript> for MapChild {
    fn from(child: SupScript) -> Self {
        MapChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for MapChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        MapChild::SupScript(builder.build())
    }
}
impl From<Table> for MapChild {
    fn from(child: Table) -> Self {
        MapChild::Table(child)
    }
}
impl From<builders::TableBuilder> for MapChild {
    fn from(builder: builders::TableBuilder) -> Self {
        MapChild::Table(builder.build())
    }
}
impl From<TableBody> for MapChild {
    fn from(child: TableBody) -> Self {
        MapChild::TableBody(child)
    }
}
impl From<builders::TableBodyBuilder> for MapChild {
    fn from(builder: builders::TableBodyBuilder) -> Self {
        MapChild::TableBody(builder.build())
    }
}
impl From<TableCell> for MapChild {
    fn from(child: TableCell) -> Self {
        MapChild::TableCell(child)
    }
}
impl From<builders::TableCellBuilder> for MapChild {
    fn from(builder: builders::TableCellBuilder) -> Self {
        MapChild::TableCell(builder.build())
    }
}
impl From<TableColumn> for MapChild {
    fn from(child: TableColumn) -> Self {
        MapChild::TableColumn(child)
    }
}
impl From<builders::TableColumnBuilder> for MapChild {
    fn from(builder: builders::TableColumnBuilder) -> Self {
        MapChild::TableColumn(builder.build())
    }
}
impl From<TableColumnGroup> for MapChild {
    fn from(child: TableColumnGroup) -> Self {
        MapChild::TableColumnGroup(child)
    }
}
impl From<builders::TableColumnGroupBuilder> for MapChild {
    fn from(builder: builders::TableColumnGroupBuilder) -> Self {
        MapChild::TableColumnGroup(builder.build())
    }
}
impl From<TableFoot> for MapChild {
    fn from(child: TableFoot) -> Self {
        MapChild::TableFoot(child)
    }
}
impl From<builders::TableFootBuilder> for MapChild {
    fn from(builder: builders::TableFootBuilder) -> Self {
        MapChild::TableFoot(builder.build())
    }
}
impl From<TableHead> for MapChild {
    fn from(child: TableHead) -> Self {
        MapChild::TableHead(child)
    }
}
impl From<builders::TableHeadBuilder> for MapChild {
    fn from(builder: builders::TableHeadBuilder) -> Self {
        MapChild::TableHead(builder.build())
    }
}
impl From<TableHeader> for MapChild {
    fn from(child: TableHeader) -> Self {
        MapChild::TableHeader(child)
    }
}
impl From<builders::TableHeaderBuilder> for MapChild {
    fn from(builder: builders::TableHeaderBuilder) -> Self {
        MapChild::TableHeader(builder.build())
    }
}
impl From<TableRow> for MapChild {
    fn from(child: TableRow) -> Self {
        MapChild::TableRow(child)
    }
}
impl From<builders::TableRowBuilder> for MapChild {
    fn from(builder: builders::TableRowBuilder) -> Self {
        MapChild::TableRow(builder.build())
    }
}
impl From<Template> for MapChild {
    fn from(child: Template) -> Self {
        MapChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for MapChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        MapChild::Template(builder.build())
    }
}
impl From<TextArea> for MapChild {
    fn from(child: TextArea) -> Self {
        MapChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for MapChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        MapChild::TextArea(builder.build())
    }
}
impl From<Time> for MapChild {
    fn from(child: Time) -> Self {
        MapChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for MapChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        MapChild::Time(builder.build())
    }
}
impl From<Title> for MapChild {
    fn from(child: Title) -> Self {
        MapChild::Title(child)
    }
}
impl From<builders::TitleBuilder> for MapChild {
    fn from(builder: builders::TitleBuilder) -> Self {
        MapChild::Title(builder.build())
    }
}
impl From<Track> for MapChild {
    fn from(child: Track) -> Self {
        MapChild::Track(child)
    }
}
impl From<builders::TrackBuilder> for MapChild {
    fn from(builder: builders::TrackBuilder) -> Self {
        MapChild::Track(builder.build())
    }
}
impl From<Underline> for MapChild {
    fn from(child: Underline) -> Self {
        MapChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for MapChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        MapChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for MapChild {
    fn from(child: UnorderedList) -> Self {
        MapChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for MapChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        MapChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for MapChild {
    fn from(child: Variable) -> Self {
        MapChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for MapChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        MapChild::Variable(builder.build())
    }
}
impl From<Video> for MapChild {
    fn from(child: Video) -> Self {
        MapChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for MapChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        MapChild::Video(builder.build())
    }
}
impl From<WordBreak> for MapChild {
    fn from(child: WordBreak) -> Self {
        MapChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for MapChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        MapChild::WordBreak(builder.build())
    }
}
impl std::fmt::Debug for MapChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Address(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Article(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Base(child) => std::fmt::Debug::fmt(child, f),
            MapChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            MapChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            MapChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Body(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Button(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Caption(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Code(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Data(child) => std::fmt::Debug::fmt(child, f),
            MapChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            MapChild::DescriptionDetails(child) => std::fmt::Debug::fmt(child, f),
            MapChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            MapChild::DescriptionTerm(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Details(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Division(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            MapChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            MapChild::FieldSetLegend(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            MapChild::FigureCaption(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Form(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Head(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Header(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            MapChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            MapChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Html(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Image(child) => std::fmt::Debug::fmt(child, f),
            MapChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Input(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Label(child) => std::fmt::Debug::fmt(child, f),
            MapChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Link(child) => std::fmt::Debug::fmt(child, f),
            MapChild::ListItem(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Main(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Map(child) => std::fmt::Debug::fmt(child, f),
            MapChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            MapChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Object(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Option(child) => std::fmt::Debug::fmt(child, f),
            MapChild::OptionGroup(child) => std::fmt::Debug::fmt(child, f),
            MapChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Output(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            MapChild::RubyParenthesis(child) => std::fmt::Debug::fmt(child, f),
            MapChild::RubyText(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Script(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Search(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Section(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Select(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Small(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Source(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Span(child) => std::fmt::Debug::fmt(child, f),
            MapChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Style(child) => std::fmt::Debug::fmt(child, f),
            MapChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Summary(child) => std::fmt::Debug::fmt(child, f),
            MapChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Table(child) => std::fmt::Debug::fmt(child, f),
            MapChild::TableBody(child) => std::fmt::Debug::fmt(child, f),
            MapChild::TableCell(child) => std::fmt::Debug::fmt(child, f),
            MapChild::TableColumn(child) => std::fmt::Debug::fmt(child, f),
            MapChild::TableColumnGroup(child) => std::fmt::Debug::fmt(child, f),
            MapChild::TableFoot(child) => std::fmt::Debug::fmt(child, f),
            MapChild::TableHead(child) => std::fmt::Debug::fmt(child, f),
            MapChild::TableHeader(child) => std::fmt::Debug::fmt(child, f),
            MapChild::TableRow(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Template(child) => std::fmt::Debug::fmt(child, f),
            MapChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Time(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Title(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Track(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            MapChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            MapChild::Video(child) => std::fmt::Debug::fmt(child, f),
            MapChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for MapChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            MapChild::Address(child) => std::fmt::Display::fmt(child, f),
            MapChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            MapChild::Article(child) => std::fmt::Display::fmt(child, f),
            MapChild::Aside(child) => std::fmt::Display::fmt(child, f),
            MapChild::Audio(child) => std::fmt::Display::fmt(child, f),
            MapChild::Base(child) => std::fmt::Display::fmt(child, f),
            MapChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            MapChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            MapChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            MapChild::Body(child) => std::fmt::Display::fmt(child, f),
            MapChild::Bold(child) => std::fmt::Display::fmt(child, f),
            MapChild::Button(child) => std::fmt::Display::fmt(child, f),
            MapChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            MapChild::Caption(child) => std::fmt::Display::fmt(child, f),
            MapChild::Cite(child) => std::fmt::Display::fmt(child, f),
            MapChild::Code(child) => std::fmt::Display::fmt(child, f),
            MapChild::Custom(child) => std::fmt::Display::fmt(child, f),
            MapChild::Data(child) => std::fmt::Display::fmt(child, f),
            MapChild::DataList(child) => std::fmt::Display::fmt(child, f),
            MapChild::Definition(child) => std::fmt::Display::fmt(child, f),
            MapChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            MapChild::DescriptionDetails(child) => std::fmt::Display::fmt(child, f),
            MapChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            MapChild::DescriptionTerm(child) => std::fmt::Display::fmt(child, f),
            MapChild::Details(child) => std::fmt::Display::fmt(child, f),
            MapChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            MapChild::Division(child) => std::fmt::Display::fmt(child, f),
            MapChild::Embed(child) => std::fmt::Display::fmt(child, f),
            MapChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            MapChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            MapChild::FieldSetLegend(child) => std::fmt::Display::fmt(child, f),
            MapChild::Figure(child) => std::fmt::Display::fmt(child, f),
            MapChild::FigureCaption(child) => std::fmt::Display::fmt(child, f),
            MapChild::Footer(child) => std::fmt::Display::fmt(child, f),
            MapChild::Form(child) => std::fmt::Display::fmt(child, f),
            MapChild::Head(child) => std::fmt::Display::fmt(child, f),
            MapChild::Header(child) => std::fmt::Display::fmt(child, f),
            MapChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            MapChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            MapChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            MapChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            MapChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            MapChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            MapChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            MapChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            MapChild::Html(child) => std::fmt::Display::fmt(child, f),
            MapChild::Image(child) => std::fmt::Display::fmt(child, f),
            MapChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            MapChild::Input(child) => std::fmt::Display::fmt(child, f),
            MapChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            MapChild::Italic(child) => std::fmt::Display::fmt(child, f),
            MapChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            MapChild::Label(child) => std::fmt::Display::fmt(child, f),
            MapChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            MapChild::Link(child) => std::fmt::Display::fmt(child, f),
            MapChild::ListItem(child) => std::fmt::Display::fmt(child, f),
            MapChild::Main(child) => std::fmt::Display::fmt(child, f),
            MapChild::Map(child) => std::fmt::Display::fmt(child, f),
            MapChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            MapChild::Mark(child) => std::fmt::Display::fmt(child, f),
            MapChild::Menu(child) => std::fmt::Display::fmt(child, f),
            MapChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            MapChild::Meter(child) => std::fmt::Display::fmt(child, f),
            MapChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            MapChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            MapChild::Object(child) => std::fmt::Display::fmt(child, f),
            MapChild::Option(child) => std::fmt::Display::fmt(child, f),
            MapChild::OptionGroup(child) => std::fmt::Display::fmt(child, f),
            MapChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            MapChild::Output(child) => std::fmt::Display::fmt(child, f),
            MapChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            MapChild::Picture(child) => std::fmt::Display::fmt(child, f),
            MapChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            MapChild::Progress(child) => std::fmt::Display::fmt(child, f),
            MapChild::Quote(child) => std::fmt::Display::fmt(child, f),
            MapChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            MapChild::RubyParenthesis(child) => std::fmt::Display::fmt(child, f),
            MapChild::RubyText(child) => std::fmt::Display::fmt(child, f),
            MapChild::Sample(child) => std::fmt::Display::fmt(child, f),
            MapChild::Script(child) => std::fmt::Display::fmt(child, f),
            MapChild::Search(child) => std::fmt::Display::fmt(child, f),
            MapChild::Section(child) => std::fmt::Display::fmt(child, f),
            MapChild::Select(child) => std::fmt::Display::fmt(child, f),
            MapChild::Slot(child) => std::fmt::Display::fmt(child, f),
            MapChild::Small(child) => std::fmt::Display::fmt(child, f),
            MapChild::Source(child) => std::fmt::Display::fmt(child, f),
            MapChild::Span(child) => std::fmt::Display::fmt(child, f),
            MapChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            MapChild::Strong(child) => std::fmt::Display::fmt(child, f),
            MapChild::Style(child) => std::fmt::Display::fmt(child, f),
            MapChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            MapChild::Summary(child) => std::fmt::Display::fmt(child, f),
            MapChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            MapChild::Table(child) => std::fmt::Display::fmt(child, f),
            MapChild::TableBody(child) => std::fmt::Display::fmt(child, f),
            MapChild::TableCell(child) => std::fmt::Display::fmt(child, f),
            MapChild::TableColumn(child) => std::fmt::Display::fmt(child, f),
            MapChild::TableColumnGroup(child) => std::fmt::Display::fmt(child, f),
            MapChild::TableFoot(child) => std::fmt::Display::fmt(child, f),
            MapChild::TableHead(child) => std::fmt::Display::fmt(child, f),
            MapChild::TableHeader(child) => std::fmt::Display::fmt(child, f),
            MapChild::TableRow(child) => std::fmt::Display::fmt(child, f),
            MapChild::Template(child) => std::fmt::Display::fmt(child, f),
            MapChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            MapChild::Time(child) => std::fmt::Display::fmt(child, f),
            MapChild::Title(child) => std::fmt::Display::fmt(child, f),
            MapChild::Track(child) => std::fmt::Display::fmt(child, f),
            MapChild::Underline(child) => std::fmt::Display::fmt(child, f),
            MapChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            MapChild::Variable(child) => std::fmt::Display::fmt(child, f),
            MapChild::Video(child) => std::fmt::Display::fmt(child, f),
            MapChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
