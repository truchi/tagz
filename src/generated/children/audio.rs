// 🤖 This file is generated!

use crate::*;
/// The `<audio>` element's children.
#[derive(Clone)]
pub enum AudioChild {
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
impl From<Abbreviation> for AudioChild {
    fn from(child: Abbreviation) -> Self {
        AudioChild::Abbreviation(child)
    }
}
impl From<builders::AbbreviationBuilder> for AudioChild {
    fn from(builder: builders::AbbreviationBuilder) -> Self {
        AudioChild::Abbreviation(builder.build())
    }
}
impl From<Address> for AudioChild {
    fn from(child: Address) -> Self {
        AudioChild::Address(child)
    }
}
impl From<builders::AddressBuilder> for AudioChild {
    fn from(builder: builders::AddressBuilder) -> Self {
        AudioChild::Address(builder.build())
    }
}
impl From<Anchor> for AudioChild {
    fn from(child: Anchor) -> Self {
        AudioChild::Anchor(child)
    }
}
impl From<builders::AnchorBuilder> for AudioChild {
    fn from(builder: builders::AnchorBuilder) -> Self {
        AudioChild::Anchor(builder.build())
    }
}
impl From<Article> for AudioChild {
    fn from(child: Article) -> Self {
        AudioChild::Article(child)
    }
}
impl From<builders::ArticleBuilder> for AudioChild {
    fn from(builder: builders::ArticleBuilder) -> Self {
        AudioChild::Article(builder.build())
    }
}
impl From<Aside> for AudioChild {
    fn from(child: Aside) -> Self {
        AudioChild::Aside(child)
    }
}
impl From<builders::AsideBuilder> for AudioChild {
    fn from(builder: builders::AsideBuilder) -> Self {
        AudioChild::Aside(builder.build())
    }
}
impl From<Audio> for AudioChild {
    fn from(child: Audio) -> Self {
        AudioChild::Audio(child)
    }
}
impl From<builders::AudioBuilder> for AudioChild {
    fn from(builder: builders::AudioBuilder) -> Self {
        AudioChild::Audio(builder.build())
    }
}
impl From<Base> for AudioChild {
    fn from(child: Base) -> Self {
        AudioChild::Base(child)
    }
}
impl From<builders::BaseBuilder> for AudioChild {
    fn from(builder: builders::BaseBuilder) -> Self {
        AudioChild::Base(builder.build())
    }
}
impl From<BidirectionalIsolate> for AudioChild {
    fn from(child: BidirectionalIsolate) -> Self {
        AudioChild::BidirectionalIsolate(child)
    }
}
impl From<builders::BidirectionalIsolateBuilder> for AudioChild {
    fn from(builder: builders::BidirectionalIsolateBuilder) -> Self {
        AudioChild::BidirectionalIsolate(builder.build())
    }
}
impl From<BidirectionalOverride> for AudioChild {
    fn from(child: BidirectionalOverride) -> Self {
        AudioChild::BidirectionalOverride(child)
    }
}
impl From<builders::BidirectionalOverrideBuilder> for AudioChild {
    fn from(builder: builders::BidirectionalOverrideBuilder) -> Self {
        AudioChild::BidirectionalOverride(builder.build())
    }
}
impl From<BlockQuote> for AudioChild {
    fn from(child: BlockQuote) -> Self {
        AudioChild::BlockQuote(child)
    }
}
impl From<builders::BlockQuoteBuilder> for AudioChild {
    fn from(builder: builders::BlockQuoteBuilder) -> Self {
        AudioChild::BlockQuote(builder.build())
    }
}
impl From<Body> for AudioChild {
    fn from(child: Body) -> Self {
        AudioChild::Body(child)
    }
}
impl From<builders::BodyBuilder> for AudioChild {
    fn from(builder: builders::BodyBuilder) -> Self {
        AudioChild::Body(builder.build())
    }
}
impl From<Bold> for AudioChild {
    fn from(child: Bold) -> Self {
        AudioChild::Bold(child)
    }
}
impl From<builders::BoldBuilder> for AudioChild {
    fn from(builder: builders::BoldBuilder) -> Self {
        AudioChild::Bold(builder.build())
    }
}
impl From<Button> for AudioChild {
    fn from(child: Button) -> Self {
        AudioChild::Button(child)
    }
}
impl From<builders::ButtonBuilder> for AudioChild {
    fn from(builder: builders::ButtonBuilder) -> Self {
        AudioChild::Button(builder.build())
    }
}
impl From<Canvas> for AudioChild {
    fn from(child: Canvas) -> Self {
        AudioChild::Canvas(child)
    }
}
impl From<builders::CanvasBuilder> for AudioChild {
    fn from(builder: builders::CanvasBuilder) -> Self {
        AudioChild::Canvas(builder.build())
    }
}
impl From<Caption> for AudioChild {
    fn from(child: Caption) -> Self {
        AudioChild::Caption(child)
    }
}
impl From<builders::CaptionBuilder> for AudioChild {
    fn from(builder: builders::CaptionBuilder) -> Self {
        AudioChild::Caption(builder.build())
    }
}
impl From<Cite> for AudioChild {
    fn from(child: Cite) -> Self {
        AudioChild::Cite(child)
    }
}
impl From<builders::CiteBuilder> for AudioChild {
    fn from(builder: builders::CiteBuilder) -> Self {
        AudioChild::Cite(builder.build())
    }
}
impl From<Code> for AudioChild {
    fn from(child: Code) -> Self {
        AudioChild::Code(child)
    }
}
impl From<builders::CodeBuilder> for AudioChild {
    fn from(builder: builders::CodeBuilder) -> Self {
        AudioChild::Code(builder.build())
    }
}
impl From<Custom> for AudioChild {
    fn from(child: Custom) -> Self {
        AudioChild::Custom(child)
    }
}
impl From<builders::CustomBuilder> for AudioChild {
    fn from(builder: builders::CustomBuilder) -> Self {
        AudioChild::Custom(builder.build())
    }
}
impl From<Data> for AudioChild {
    fn from(child: Data) -> Self {
        AudioChild::Data(child)
    }
}
impl From<builders::DataBuilder> for AudioChild {
    fn from(builder: builders::DataBuilder) -> Self {
        AudioChild::Data(builder.build())
    }
}
impl From<DataList> for AudioChild {
    fn from(child: DataList) -> Self {
        AudioChild::DataList(child)
    }
}
impl From<builders::DataListBuilder> for AudioChild {
    fn from(builder: builders::DataListBuilder) -> Self {
        AudioChild::DataList(builder.build())
    }
}
impl From<Definition> for AudioChild {
    fn from(child: Definition) -> Self {
        AudioChild::Definition(child)
    }
}
impl From<builders::DefinitionBuilder> for AudioChild {
    fn from(builder: builders::DefinitionBuilder) -> Self {
        AudioChild::Definition(builder.build())
    }
}
impl From<Deleted> for AudioChild {
    fn from(child: Deleted) -> Self {
        AudioChild::Deleted(child)
    }
}
impl From<builders::DeletedBuilder> for AudioChild {
    fn from(builder: builders::DeletedBuilder) -> Self {
        AudioChild::Deleted(builder.build())
    }
}
impl From<DescriptionDetails> for AudioChild {
    fn from(child: DescriptionDetails) -> Self {
        AudioChild::DescriptionDetails(child)
    }
}
impl From<builders::DescriptionDetailsBuilder> for AudioChild {
    fn from(builder: builders::DescriptionDetailsBuilder) -> Self {
        AudioChild::DescriptionDetails(builder.build())
    }
}
impl From<DescriptionList> for AudioChild {
    fn from(child: DescriptionList) -> Self {
        AudioChild::DescriptionList(child)
    }
}
impl From<builders::DescriptionListBuilder> for AudioChild {
    fn from(builder: builders::DescriptionListBuilder) -> Self {
        AudioChild::DescriptionList(builder.build())
    }
}
impl From<DescriptionTerm> for AudioChild {
    fn from(child: DescriptionTerm) -> Self {
        AudioChild::DescriptionTerm(child)
    }
}
impl From<builders::DescriptionTermBuilder> for AudioChild {
    fn from(builder: builders::DescriptionTermBuilder) -> Self {
        AudioChild::DescriptionTerm(builder.build())
    }
}
impl From<Details> for AudioChild {
    fn from(child: Details) -> Self {
        AudioChild::Details(child)
    }
}
impl From<builders::DetailsBuilder> for AudioChild {
    fn from(builder: builders::DetailsBuilder) -> Self {
        AudioChild::Details(builder.build())
    }
}
impl From<Dialog> for AudioChild {
    fn from(child: Dialog) -> Self {
        AudioChild::Dialog(child)
    }
}
impl From<builders::DialogBuilder> for AudioChild {
    fn from(builder: builders::DialogBuilder) -> Self {
        AudioChild::Dialog(builder.build())
    }
}
impl From<Division> for AudioChild {
    fn from(child: Division) -> Self {
        AudioChild::Division(child)
    }
}
impl From<builders::DivisionBuilder> for AudioChild {
    fn from(builder: builders::DivisionBuilder) -> Self {
        AudioChild::Division(builder.build())
    }
}
impl From<Embed> for AudioChild {
    fn from(child: Embed) -> Self {
        AudioChild::Embed(child)
    }
}
impl From<builders::EmbedBuilder> for AudioChild {
    fn from(builder: builders::EmbedBuilder) -> Self {
        AudioChild::Embed(builder.build())
    }
}
impl From<Emphasis> for AudioChild {
    fn from(child: Emphasis) -> Self {
        AudioChild::Emphasis(child)
    }
}
impl From<builders::EmphasisBuilder> for AudioChild {
    fn from(builder: builders::EmphasisBuilder) -> Self {
        AudioChild::Emphasis(builder.build())
    }
}
impl From<FieldSet> for AudioChild {
    fn from(child: FieldSet) -> Self {
        AudioChild::FieldSet(child)
    }
}
impl From<builders::FieldSetBuilder> for AudioChild {
    fn from(builder: builders::FieldSetBuilder) -> Self {
        AudioChild::FieldSet(builder.build())
    }
}
impl From<FieldSetLegend> for AudioChild {
    fn from(child: FieldSetLegend) -> Self {
        AudioChild::FieldSetLegend(child)
    }
}
impl From<builders::FieldSetLegendBuilder> for AudioChild {
    fn from(builder: builders::FieldSetLegendBuilder) -> Self {
        AudioChild::FieldSetLegend(builder.build())
    }
}
impl From<Figure> for AudioChild {
    fn from(child: Figure) -> Self {
        AudioChild::Figure(child)
    }
}
impl From<builders::FigureBuilder> for AudioChild {
    fn from(builder: builders::FigureBuilder) -> Self {
        AudioChild::Figure(builder.build())
    }
}
impl From<FigureCaption> for AudioChild {
    fn from(child: FigureCaption) -> Self {
        AudioChild::FigureCaption(child)
    }
}
impl From<builders::FigureCaptionBuilder> for AudioChild {
    fn from(builder: builders::FigureCaptionBuilder) -> Self {
        AudioChild::FigureCaption(builder.build())
    }
}
impl From<Footer> for AudioChild {
    fn from(child: Footer) -> Self {
        AudioChild::Footer(child)
    }
}
impl From<builders::FooterBuilder> for AudioChild {
    fn from(builder: builders::FooterBuilder) -> Self {
        AudioChild::Footer(builder.build())
    }
}
impl From<Form> for AudioChild {
    fn from(child: Form) -> Self {
        AudioChild::Form(child)
    }
}
impl From<builders::FormBuilder> for AudioChild {
    fn from(builder: builders::FormBuilder) -> Self {
        AudioChild::Form(builder.build())
    }
}
impl From<Head> for AudioChild {
    fn from(child: Head) -> Self {
        AudioChild::Head(child)
    }
}
impl From<builders::HeadBuilder> for AudioChild {
    fn from(builder: builders::HeadBuilder) -> Self {
        AudioChild::Head(builder.build())
    }
}
impl From<Header> for AudioChild {
    fn from(child: Header) -> Self {
        AudioChild::Header(child)
    }
}
impl From<builders::HeaderBuilder> for AudioChild {
    fn from(builder: builders::HeaderBuilder) -> Self {
        AudioChild::Header(builder.build())
    }
}
impl From<Heading1> for AudioChild {
    fn from(child: Heading1) -> Self {
        AudioChild::Heading1(child)
    }
}
impl From<builders::Heading1Builder> for AudioChild {
    fn from(builder: builders::Heading1Builder) -> Self {
        AudioChild::Heading1(builder.build())
    }
}
impl From<Heading2> for AudioChild {
    fn from(child: Heading2) -> Self {
        AudioChild::Heading2(child)
    }
}
impl From<builders::Heading2Builder> for AudioChild {
    fn from(builder: builders::Heading2Builder) -> Self {
        AudioChild::Heading2(builder.build())
    }
}
impl From<Heading3> for AudioChild {
    fn from(child: Heading3) -> Self {
        AudioChild::Heading3(child)
    }
}
impl From<builders::Heading3Builder> for AudioChild {
    fn from(builder: builders::Heading3Builder) -> Self {
        AudioChild::Heading3(builder.build())
    }
}
impl From<Heading4> for AudioChild {
    fn from(child: Heading4) -> Self {
        AudioChild::Heading4(child)
    }
}
impl From<builders::Heading4Builder> for AudioChild {
    fn from(builder: builders::Heading4Builder) -> Self {
        AudioChild::Heading4(builder.build())
    }
}
impl From<Heading5> for AudioChild {
    fn from(child: Heading5) -> Self {
        AudioChild::Heading5(child)
    }
}
impl From<builders::Heading5Builder> for AudioChild {
    fn from(builder: builders::Heading5Builder) -> Self {
        AudioChild::Heading5(builder.build())
    }
}
impl From<Heading6> for AudioChild {
    fn from(child: Heading6) -> Self {
        AudioChild::Heading6(child)
    }
}
impl From<builders::Heading6Builder> for AudioChild {
    fn from(builder: builders::Heading6Builder) -> Self {
        AudioChild::Heading6(builder.build())
    }
}
impl From<HeadingGroup> for AudioChild {
    fn from(child: HeadingGroup) -> Self {
        AudioChild::HeadingGroup(child)
    }
}
impl From<builders::HeadingGroupBuilder> for AudioChild {
    fn from(builder: builders::HeadingGroupBuilder) -> Self {
        AudioChild::HeadingGroup(builder.build())
    }
}
impl From<HorizontalRule> for AudioChild {
    fn from(child: HorizontalRule) -> Self {
        AudioChild::HorizontalRule(child)
    }
}
impl From<builders::HorizontalRuleBuilder> for AudioChild {
    fn from(builder: builders::HorizontalRuleBuilder) -> Self {
        AudioChild::HorizontalRule(builder.build())
    }
}
impl From<Html> for AudioChild {
    fn from(child: Html) -> Self {
        AudioChild::Html(child)
    }
}
impl From<builders::HtmlBuilder> for AudioChild {
    fn from(builder: builders::HtmlBuilder) -> Self {
        AudioChild::Html(builder.build())
    }
}
impl From<Image> for AudioChild {
    fn from(child: Image) -> Self {
        AudioChild::Image(child)
    }
}
impl From<builders::ImageBuilder> for AudioChild {
    fn from(builder: builders::ImageBuilder) -> Self {
        AudioChild::Image(builder.build())
    }
}
impl From<InlineFrame> for AudioChild {
    fn from(child: InlineFrame) -> Self {
        AudioChild::InlineFrame(child)
    }
}
impl From<builders::InlineFrameBuilder> for AudioChild {
    fn from(builder: builders::InlineFrameBuilder) -> Self {
        AudioChild::InlineFrame(builder.build())
    }
}
impl From<Input> for AudioChild {
    fn from(child: Input) -> Self {
        AudioChild::Input(child)
    }
}
impl From<builders::InputBuilder> for AudioChild {
    fn from(builder: builders::InputBuilder) -> Self {
        AudioChild::Input(builder.build())
    }
}
impl From<Inserted> for AudioChild {
    fn from(child: Inserted) -> Self {
        AudioChild::Inserted(child)
    }
}
impl From<builders::InsertedBuilder> for AudioChild {
    fn from(builder: builders::InsertedBuilder) -> Self {
        AudioChild::Inserted(builder.build())
    }
}
impl From<Italic> for AudioChild {
    fn from(child: Italic) -> Self {
        AudioChild::Italic(child)
    }
}
impl From<builders::ItalicBuilder> for AudioChild {
    fn from(builder: builders::ItalicBuilder) -> Self {
        AudioChild::Italic(builder.build())
    }
}
impl From<Keyboard> for AudioChild {
    fn from(child: Keyboard) -> Self {
        AudioChild::Keyboard(child)
    }
}
impl From<builders::KeyboardBuilder> for AudioChild {
    fn from(builder: builders::KeyboardBuilder) -> Self {
        AudioChild::Keyboard(builder.build())
    }
}
impl From<Label> for AudioChild {
    fn from(child: Label) -> Self {
        AudioChild::Label(child)
    }
}
impl From<builders::LabelBuilder> for AudioChild {
    fn from(builder: builders::LabelBuilder) -> Self {
        AudioChild::Label(builder.build())
    }
}
impl From<LineBreak> for AudioChild {
    fn from(child: LineBreak) -> Self {
        AudioChild::LineBreak(child)
    }
}
impl From<builders::LineBreakBuilder> for AudioChild {
    fn from(builder: builders::LineBreakBuilder) -> Self {
        AudioChild::LineBreak(builder.build())
    }
}
impl From<Link> for AudioChild {
    fn from(child: Link) -> Self {
        AudioChild::Link(child)
    }
}
impl From<builders::LinkBuilder> for AudioChild {
    fn from(builder: builders::LinkBuilder) -> Self {
        AudioChild::Link(builder.build())
    }
}
impl From<ListItem> for AudioChild {
    fn from(child: ListItem) -> Self {
        AudioChild::ListItem(child)
    }
}
impl From<builders::ListItemBuilder> for AudioChild {
    fn from(builder: builders::ListItemBuilder) -> Self {
        AudioChild::ListItem(builder.build())
    }
}
impl From<Main> for AudioChild {
    fn from(child: Main) -> Self {
        AudioChild::Main(child)
    }
}
impl From<builders::MainBuilder> for AudioChild {
    fn from(builder: builders::MainBuilder) -> Self {
        AudioChild::Main(builder.build())
    }
}
impl From<Map> for AudioChild {
    fn from(child: Map) -> Self {
        AudioChild::Map(child)
    }
}
impl From<builders::MapBuilder> for AudioChild {
    fn from(builder: builders::MapBuilder) -> Self {
        AudioChild::Map(builder.build())
    }
}
impl From<MapArea> for AudioChild {
    fn from(child: MapArea) -> Self {
        AudioChild::MapArea(child)
    }
}
impl From<builders::MapAreaBuilder> for AudioChild {
    fn from(builder: builders::MapAreaBuilder) -> Self {
        AudioChild::MapArea(builder.build())
    }
}
impl From<Mark> for AudioChild {
    fn from(child: Mark) -> Self {
        AudioChild::Mark(child)
    }
}
impl From<builders::MarkBuilder> for AudioChild {
    fn from(builder: builders::MarkBuilder) -> Self {
        AudioChild::Mark(builder.build())
    }
}
impl From<Menu> for AudioChild {
    fn from(child: Menu) -> Self {
        AudioChild::Menu(child)
    }
}
impl From<builders::MenuBuilder> for AudioChild {
    fn from(builder: builders::MenuBuilder) -> Self {
        AudioChild::Menu(builder.build())
    }
}
impl From<Metadata> for AudioChild {
    fn from(child: Metadata) -> Self {
        AudioChild::Metadata(child)
    }
}
impl From<builders::MetadataBuilder> for AudioChild {
    fn from(builder: builders::MetadataBuilder) -> Self {
        AudioChild::Metadata(builder.build())
    }
}
impl From<Meter> for AudioChild {
    fn from(child: Meter) -> Self {
        AudioChild::Meter(child)
    }
}
impl From<builders::MeterBuilder> for AudioChild {
    fn from(builder: builders::MeterBuilder) -> Self {
        AudioChild::Meter(builder.build())
    }
}
impl From<Navigation> for AudioChild {
    fn from(child: Navigation) -> Self {
        AudioChild::Navigation(child)
    }
}
impl From<builders::NavigationBuilder> for AudioChild {
    fn from(builder: builders::NavigationBuilder) -> Self {
        AudioChild::Navigation(builder.build())
    }
}
impl From<NoScript> for AudioChild {
    fn from(child: NoScript) -> Self {
        AudioChild::NoScript(child)
    }
}
impl From<builders::NoScriptBuilder> for AudioChild {
    fn from(builder: builders::NoScriptBuilder) -> Self {
        AudioChild::NoScript(builder.build())
    }
}
impl From<Object> for AudioChild {
    fn from(child: Object) -> Self {
        AudioChild::Object(child)
    }
}
impl From<builders::ObjectBuilder> for AudioChild {
    fn from(builder: builders::ObjectBuilder) -> Self {
        AudioChild::Object(builder.build())
    }
}
impl From<Option> for AudioChild {
    fn from(child: Option) -> Self {
        AudioChild::Option(child)
    }
}
impl From<builders::OptionBuilder> for AudioChild {
    fn from(builder: builders::OptionBuilder) -> Self {
        AudioChild::Option(builder.build())
    }
}
impl From<OptionGroup> for AudioChild {
    fn from(child: OptionGroup) -> Self {
        AudioChild::OptionGroup(child)
    }
}
impl From<builders::OptionGroupBuilder> for AudioChild {
    fn from(builder: builders::OptionGroupBuilder) -> Self {
        AudioChild::OptionGroup(builder.build())
    }
}
impl From<OrderedList> for AudioChild {
    fn from(child: OrderedList) -> Self {
        AudioChild::OrderedList(child)
    }
}
impl From<builders::OrderedListBuilder> for AudioChild {
    fn from(builder: builders::OrderedListBuilder) -> Self {
        AudioChild::OrderedList(builder.build())
    }
}
impl From<Output> for AudioChild {
    fn from(child: Output) -> Self {
        AudioChild::Output(child)
    }
}
impl From<builders::OutputBuilder> for AudioChild {
    fn from(builder: builders::OutputBuilder) -> Self {
        AudioChild::Output(builder.build())
    }
}
impl From<Paragraph> for AudioChild {
    fn from(child: Paragraph) -> Self {
        AudioChild::Paragraph(child)
    }
}
impl From<builders::ParagraphBuilder> for AudioChild {
    fn from(builder: builders::ParagraphBuilder) -> Self {
        AudioChild::Paragraph(builder.build())
    }
}
impl From<Picture> for AudioChild {
    fn from(child: Picture) -> Self {
        AudioChild::Picture(child)
    }
}
impl From<builders::PictureBuilder> for AudioChild {
    fn from(builder: builders::PictureBuilder) -> Self {
        AudioChild::Picture(builder.build())
    }
}
impl From<Preformatted> for AudioChild {
    fn from(child: Preformatted) -> Self {
        AudioChild::Preformatted(child)
    }
}
impl From<builders::PreformattedBuilder> for AudioChild {
    fn from(builder: builders::PreformattedBuilder) -> Self {
        AudioChild::Preformatted(builder.build())
    }
}
impl From<Progress> for AudioChild {
    fn from(child: Progress) -> Self {
        AudioChild::Progress(child)
    }
}
impl From<builders::ProgressBuilder> for AudioChild {
    fn from(builder: builders::ProgressBuilder) -> Self {
        AudioChild::Progress(builder.build())
    }
}
impl From<Quote> for AudioChild {
    fn from(child: Quote) -> Self {
        AudioChild::Quote(child)
    }
}
impl From<builders::QuoteBuilder> for AudioChild {
    fn from(builder: builders::QuoteBuilder) -> Self {
        AudioChild::Quote(builder.build())
    }
}
impl From<Ruby> for AudioChild {
    fn from(child: Ruby) -> Self {
        AudioChild::Ruby(child)
    }
}
impl From<builders::RubyBuilder> for AudioChild {
    fn from(builder: builders::RubyBuilder) -> Self {
        AudioChild::Ruby(builder.build())
    }
}
impl From<RubyParenthesis> for AudioChild {
    fn from(child: RubyParenthesis) -> Self {
        AudioChild::RubyParenthesis(child)
    }
}
impl From<builders::RubyParenthesisBuilder> for AudioChild {
    fn from(builder: builders::RubyParenthesisBuilder) -> Self {
        AudioChild::RubyParenthesis(builder.build())
    }
}
impl From<RubyText> for AudioChild {
    fn from(child: RubyText) -> Self {
        AudioChild::RubyText(child)
    }
}
impl From<builders::RubyTextBuilder> for AudioChild {
    fn from(builder: builders::RubyTextBuilder) -> Self {
        AudioChild::RubyText(builder.build())
    }
}
impl From<Sample> for AudioChild {
    fn from(child: Sample) -> Self {
        AudioChild::Sample(child)
    }
}
impl From<builders::SampleBuilder> for AudioChild {
    fn from(builder: builders::SampleBuilder) -> Self {
        AudioChild::Sample(builder.build())
    }
}
impl From<Script> for AudioChild {
    fn from(child: Script) -> Self {
        AudioChild::Script(child)
    }
}
impl From<builders::ScriptBuilder> for AudioChild {
    fn from(builder: builders::ScriptBuilder) -> Self {
        AudioChild::Script(builder.build())
    }
}
impl From<Search> for AudioChild {
    fn from(child: Search) -> Self {
        AudioChild::Search(child)
    }
}
impl From<builders::SearchBuilder> for AudioChild {
    fn from(builder: builders::SearchBuilder) -> Self {
        AudioChild::Search(builder.build())
    }
}
impl From<Section> for AudioChild {
    fn from(child: Section) -> Self {
        AudioChild::Section(child)
    }
}
impl From<builders::SectionBuilder> for AudioChild {
    fn from(builder: builders::SectionBuilder) -> Self {
        AudioChild::Section(builder.build())
    }
}
impl From<Select> for AudioChild {
    fn from(child: Select) -> Self {
        AudioChild::Select(child)
    }
}
impl From<builders::SelectBuilder> for AudioChild {
    fn from(builder: builders::SelectBuilder) -> Self {
        AudioChild::Select(builder.build())
    }
}
impl From<Slot> for AudioChild {
    fn from(child: Slot) -> Self {
        AudioChild::Slot(child)
    }
}
impl From<builders::SlotBuilder> for AudioChild {
    fn from(builder: builders::SlotBuilder) -> Self {
        AudioChild::Slot(builder.build())
    }
}
impl From<Small> for AudioChild {
    fn from(child: Small) -> Self {
        AudioChild::Small(child)
    }
}
impl From<builders::SmallBuilder> for AudioChild {
    fn from(builder: builders::SmallBuilder) -> Self {
        AudioChild::Small(builder.build())
    }
}
impl From<Source> for AudioChild {
    fn from(child: Source) -> Self {
        AudioChild::Source(child)
    }
}
impl From<builders::SourceBuilder> for AudioChild {
    fn from(builder: builders::SourceBuilder) -> Self {
        AudioChild::Source(builder.build())
    }
}
impl From<Span> for AudioChild {
    fn from(child: Span) -> Self {
        AudioChild::Span(child)
    }
}
impl From<builders::SpanBuilder> for AudioChild {
    fn from(builder: builders::SpanBuilder) -> Self {
        AudioChild::Span(builder.build())
    }
}
impl From<StrikeThrough> for AudioChild {
    fn from(child: StrikeThrough) -> Self {
        AudioChild::StrikeThrough(child)
    }
}
impl From<builders::StrikeThroughBuilder> for AudioChild {
    fn from(builder: builders::StrikeThroughBuilder) -> Self {
        AudioChild::StrikeThrough(builder.build())
    }
}
impl From<Strong> for AudioChild {
    fn from(child: Strong) -> Self {
        AudioChild::Strong(child)
    }
}
impl From<builders::StrongBuilder> for AudioChild {
    fn from(builder: builders::StrongBuilder) -> Self {
        AudioChild::Strong(builder.build())
    }
}
impl From<Style> for AudioChild {
    fn from(child: Style) -> Self {
        AudioChild::Style(child)
    }
}
impl From<builders::StyleBuilder> for AudioChild {
    fn from(builder: builders::StyleBuilder) -> Self {
        AudioChild::Style(builder.build())
    }
}
impl From<SubScript> for AudioChild {
    fn from(child: SubScript) -> Self {
        AudioChild::SubScript(child)
    }
}
impl From<builders::SubScriptBuilder> for AudioChild {
    fn from(builder: builders::SubScriptBuilder) -> Self {
        AudioChild::SubScript(builder.build())
    }
}
impl From<Summary> for AudioChild {
    fn from(child: Summary) -> Self {
        AudioChild::Summary(child)
    }
}
impl From<builders::SummaryBuilder> for AudioChild {
    fn from(builder: builders::SummaryBuilder) -> Self {
        AudioChild::Summary(builder.build())
    }
}
impl From<SupScript> for AudioChild {
    fn from(child: SupScript) -> Self {
        AudioChild::SupScript(child)
    }
}
impl From<builders::SupScriptBuilder> for AudioChild {
    fn from(builder: builders::SupScriptBuilder) -> Self {
        AudioChild::SupScript(builder.build())
    }
}
impl From<Table> for AudioChild {
    fn from(child: Table) -> Self {
        AudioChild::Table(child)
    }
}
impl From<builders::TableBuilder> for AudioChild {
    fn from(builder: builders::TableBuilder) -> Self {
        AudioChild::Table(builder.build())
    }
}
impl From<TableBody> for AudioChild {
    fn from(child: TableBody) -> Self {
        AudioChild::TableBody(child)
    }
}
impl From<builders::TableBodyBuilder> for AudioChild {
    fn from(builder: builders::TableBodyBuilder) -> Self {
        AudioChild::TableBody(builder.build())
    }
}
impl From<TableCell> for AudioChild {
    fn from(child: TableCell) -> Self {
        AudioChild::TableCell(child)
    }
}
impl From<builders::TableCellBuilder> for AudioChild {
    fn from(builder: builders::TableCellBuilder) -> Self {
        AudioChild::TableCell(builder.build())
    }
}
impl From<TableColumn> for AudioChild {
    fn from(child: TableColumn) -> Self {
        AudioChild::TableColumn(child)
    }
}
impl From<builders::TableColumnBuilder> for AudioChild {
    fn from(builder: builders::TableColumnBuilder) -> Self {
        AudioChild::TableColumn(builder.build())
    }
}
impl From<TableColumnGroup> for AudioChild {
    fn from(child: TableColumnGroup) -> Self {
        AudioChild::TableColumnGroup(child)
    }
}
impl From<builders::TableColumnGroupBuilder> for AudioChild {
    fn from(builder: builders::TableColumnGroupBuilder) -> Self {
        AudioChild::TableColumnGroup(builder.build())
    }
}
impl From<TableFoot> for AudioChild {
    fn from(child: TableFoot) -> Self {
        AudioChild::TableFoot(child)
    }
}
impl From<builders::TableFootBuilder> for AudioChild {
    fn from(builder: builders::TableFootBuilder) -> Self {
        AudioChild::TableFoot(builder.build())
    }
}
impl From<TableHead> for AudioChild {
    fn from(child: TableHead) -> Self {
        AudioChild::TableHead(child)
    }
}
impl From<builders::TableHeadBuilder> for AudioChild {
    fn from(builder: builders::TableHeadBuilder) -> Self {
        AudioChild::TableHead(builder.build())
    }
}
impl From<TableHeader> for AudioChild {
    fn from(child: TableHeader) -> Self {
        AudioChild::TableHeader(child)
    }
}
impl From<builders::TableHeaderBuilder> for AudioChild {
    fn from(builder: builders::TableHeaderBuilder) -> Self {
        AudioChild::TableHeader(builder.build())
    }
}
impl From<TableRow> for AudioChild {
    fn from(child: TableRow) -> Self {
        AudioChild::TableRow(child)
    }
}
impl From<builders::TableRowBuilder> for AudioChild {
    fn from(builder: builders::TableRowBuilder) -> Self {
        AudioChild::TableRow(builder.build())
    }
}
impl From<Template> for AudioChild {
    fn from(child: Template) -> Self {
        AudioChild::Template(child)
    }
}
impl From<builders::TemplateBuilder> for AudioChild {
    fn from(builder: builders::TemplateBuilder) -> Self {
        AudioChild::Template(builder.build())
    }
}
impl From<TextArea> for AudioChild {
    fn from(child: TextArea) -> Self {
        AudioChild::TextArea(child)
    }
}
impl From<builders::TextAreaBuilder> for AudioChild {
    fn from(builder: builders::TextAreaBuilder) -> Self {
        AudioChild::TextArea(builder.build())
    }
}
impl From<Time> for AudioChild {
    fn from(child: Time) -> Self {
        AudioChild::Time(child)
    }
}
impl From<builders::TimeBuilder> for AudioChild {
    fn from(builder: builders::TimeBuilder) -> Self {
        AudioChild::Time(builder.build())
    }
}
impl From<Title> for AudioChild {
    fn from(child: Title) -> Self {
        AudioChild::Title(child)
    }
}
impl From<builders::TitleBuilder> for AudioChild {
    fn from(builder: builders::TitleBuilder) -> Self {
        AudioChild::Title(builder.build())
    }
}
impl From<Track> for AudioChild {
    fn from(child: Track) -> Self {
        AudioChild::Track(child)
    }
}
impl From<builders::TrackBuilder> for AudioChild {
    fn from(builder: builders::TrackBuilder) -> Self {
        AudioChild::Track(builder.build())
    }
}
impl From<Underline> for AudioChild {
    fn from(child: Underline) -> Self {
        AudioChild::Underline(child)
    }
}
impl From<builders::UnderlineBuilder> for AudioChild {
    fn from(builder: builders::UnderlineBuilder) -> Self {
        AudioChild::Underline(builder.build())
    }
}
impl From<UnorderedList> for AudioChild {
    fn from(child: UnorderedList) -> Self {
        AudioChild::UnorderedList(child)
    }
}
impl From<builders::UnorderedListBuilder> for AudioChild {
    fn from(builder: builders::UnorderedListBuilder) -> Self {
        AudioChild::UnorderedList(builder.build())
    }
}
impl From<Variable> for AudioChild {
    fn from(child: Variable) -> Self {
        AudioChild::Variable(child)
    }
}
impl From<builders::VariableBuilder> for AudioChild {
    fn from(builder: builders::VariableBuilder) -> Self {
        AudioChild::Variable(builder.build())
    }
}
impl From<Video> for AudioChild {
    fn from(child: Video) -> Self {
        AudioChild::Video(child)
    }
}
impl From<builders::VideoBuilder> for AudioChild {
    fn from(builder: builders::VideoBuilder) -> Self {
        AudioChild::Video(builder.build())
    }
}
impl From<WordBreak> for AudioChild {
    fn from(child: WordBreak) -> Self {
        AudioChild::WordBreak(child)
    }
}
impl From<builders::WordBreakBuilder> for AudioChild {
    fn from(builder: builders::WordBreakBuilder) -> Self {
        AudioChild::WordBreak(builder.build())
    }
}
impl std::fmt::Debug for AudioChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioChild::Abbreviation(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Address(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Anchor(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Article(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Aside(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Audio(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Base(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::BidirectionalIsolate(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::BidirectionalOverride(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::BlockQuote(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Body(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Bold(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Button(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Canvas(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Caption(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Cite(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Code(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Custom(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Data(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::DataList(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Definition(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Deleted(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::DescriptionDetails(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::DescriptionList(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::DescriptionTerm(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Details(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Dialog(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Division(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Embed(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Emphasis(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::FieldSet(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::FieldSetLegend(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Figure(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::FigureCaption(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Footer(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Form(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Head(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Header(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Heading1(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Heading2(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Heading3(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Heading4(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Heading5(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Heading6(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::HeadingGroup(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::HorizontalRule(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Html(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Image(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::InlineFrame(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Input(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Inserted(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Italic(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Keyboard(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Label(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::LineBreak(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Link(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::ListItem(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Main(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Map(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::MapArea(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Mark(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Menu(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Metadata(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Meter(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Navigation(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::NoScript(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Object(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Option(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::OptionGroup(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::OrderedList(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Output(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Paragraph(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Picture(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Preformatted(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Progress(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Quote(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Ruby(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::RubyParenthesis(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::RubyText(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Sample(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Script(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Search(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Section(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Select(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Slot(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Small(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Source(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Span(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::StrikeThrough(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Strong(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Style(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::SubScript(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Summary(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::SupScript(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Table(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::TableBody(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::TableCell(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::TableColumn(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::TableColumnGroup(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::TableFoot(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::TableHead(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::TableHeader(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::TableRow(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Template(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::TextArea(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Time(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Title(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Track(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Underline(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::UnorderedList(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Variable(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::Video(child) => std::fmt::Debug::fmt(child, f),
            AudioChild::WordBreak(child) => std::fmt::Debug::fmt(child, f),
        }
    }
}
impl std::fmt::Display for AudioChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioChild::Abbreviation(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Address(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Anchor(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Article(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Aside(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Audio(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Base(child) => std::fmt::Display::fmt(child, f),
            AudioChild::BidirectionalIsolate(child) => std::fmt::Display::fmt(child, f),
            AudioChild::BidirectionalOverride(child) => std::fmt::Display::fmt(child, f),
            AudioChild::BlockQuote(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Body(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Bold(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Button(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Canvas(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Caption(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Cite(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Code(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Custom(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Data(child) => std::fmt::Display::fmt(child, f),
            AudioChild::DataList(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Definition(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Deleted(child) => std::fmt::Display::fmt(child, f),
            AudioChild::DescriptionDetails(child) => std::fmt::Display::fmt(child, f),
            AudioChild::DescriptionList(child) => std::fmt::Display::fmt(child, f),
            AudioChild::DescriptionTerm(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Details(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Dialog(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Division(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Embed(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Emphasis(child) => std::fmt::Display::fmt(child, f),
            AudioChild::FieldSet(child) => std::fmt::Display::fmt(child, f),
            AudioChild::FieldSetLegend(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Figure(child) => std::fmt::Display::fmt(child, f),
            AudioChild::FigureCaption(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Footer(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Form(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Head(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Header(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Heading1(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Heading2(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Heading3(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Heading4(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Heading5(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Heading6(child) => std::fmt::Display::fmt(child, f),
            AudioChild::HeadingGroup(child) => std::fmt::Display::fmt(child, f),
            AudioChild::HorizontalRule(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Html(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Image(child) => std::fmt::Display::fmt(child, f),
            AudioChild::InlineFrame(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Input(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Inserted(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Italic(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Keyboard(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Label(child) => std::fmt::Display::fmt(child, f),
            AudioChild::LineBreak(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Link(child) => std::fmt::Display::fmt(child, f),
            AudioChild::ListItem(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Main(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Map(child) => std::fmt::Display::fmt(child, f),
            AudioChild::MapArea(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Mark(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Menu(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Metadata(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Meter(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Navigation(child) => std::fmt::Display::fmt(child, f),
            AudioChild::NoScript(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Object(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Option(child) => std::fmt::Display::fmt(child, f),
            AudioChild::OptionGroup(child) => std::fmt::Display::fmt(child, f),
            AudioChild::OrderedList(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Output(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Paragraph(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Picture(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Preformatted(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Progress(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Quote(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Ruby(child) => std::fmt::Display::fmt(child, f),
            AudioChild::RubyParenthesis(child) => std::fmt::Display::fmt(child, f),
            AudioChild::RubyText(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Sample(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Script(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Search(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Section(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Select(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Slot(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Small(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Source(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Span(child) => std::fmt::Display::fmt(child, f),
            AudioChild::StrikeThrough(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Strong(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Style(child) => std::fmt::Display::fmt(child, f),
            AudioChild::SubScript(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Summary(child) => std::fmt::Display::fmt(child, f),
            AudioChild::SupScript(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Table(child) => std::fmt::Display::fmt(child, f),
            AudioChild::TableBody(child) => std::fmt::Display::fmt(child, f),
            AudioChild::TableCell(child) => std::fmt::Display::fmt(child, f),
            AudioChild::TableColumn(child) => std::fmt::Display::fmt(child, f),
            AudioChild::TableColumnGroup(child) => std::fmt::Display::fmt(child, f),
            AudioChild::TableFoot(child) => std::fmt::Display::fmt(child, f),
            AudioChild::TableHead(child) => std::fmt::Display::fmt(child, f),
            AudioChild::TableHeader(child) => std::fmt::Display::fmt(child, f),
            AudioChild::TableRow(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Template(child) => std::fmt::Display::fmt(child, f),
            AudioChild::TextArea(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Time(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Title(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Track(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Underline(child) => std::fmt::Display::fmt(child, f),
            AudioChild::UnorderedList(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Variable(child) => std::fmt::Display::fmt(child, f),
            AudioChild::Video(child) => std::fmt::Display::fmt(child, f),
            AudioChild::WordBreak(child) => std::fmt::Display::fmt(child, f),
        }
    }
}
