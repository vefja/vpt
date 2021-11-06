import gi
import update as up


gi.require_version("Gtk", "3.0")
from gi.repository import Gtk

packages_list = [
    ("Elements", "Core"),
    ("Neofetch", "Extra"),
]


class TreeViewFilterWindow(Gtk.Window):
    def __init__(self):
        super().__init__(title="Nitrogen Store")
        self.set_border_width(10)

        # Setting up the self.grid in which the elements are to be positioned
        self.grid = Gtk.Grid()
        self.grid.set_column_homogeneous(True)
        self.grid.set_row_homogeneous(True)
        self.add(self.grid)

        # Creating the ListStore model
        self.ntg_store_list = Gtk.ListStore(str, str)
        for software_ref in packages_list:
            self.ntg_store_list.append(list(software_ref))
        self.current_filter_category = None

        # Creating the filter, feeding it with the liststore model
        self.category_filter = self.ntg_store_list.filter_new()
        # setting the filter function, note that we're not using the
        self.category_filter.set_visible_func(self.category_filter_func)

        # creating the treeview, making it use the filter as a model, and adding the columns
        self.treeview = Gtk.TreeView(model=self.category_filter)
        for i, column_title in enumerate(
            ["Package", "Category"]
        ):
            renderer = Gtk.CellRendererText()
            column = Gtk.TreeViewColumn(column_title, renderer, text=i)
            self.treeview.append_column(column)

        self.buttons = list()
        for pkg_category in ["Add", "Delete", "Update"]:
            button = Gtk.Button(label=pkg_category)
            self.buttons.append(button)
            button.connect("clicked", self.on_selection_button_clicked)

        self.scrollable_treelist = Gtk.ScrolledWindow()
        self.scrollable_treelist.set_vexpand(True)
        self.grid.attach(self.scrollable_treelist, 0, 0, 8, 10)
        self.grid.attach_next_to(
            self.buttons[0], self.scrollable_treelist, Gtk.PositionType.BOTTOM, 1, 1
        )
        for i, button in enumerate(self.buttons[1:]):
            self.grid.attach_next_to(
                button, self.buttons[i], Gtk.PositionType.RIGHT, 1, 1
            )
        self.scrollable_treelist.add(self.treeview)

        self.show_all()

    def category_filter_func(self, model, iter, data):
        if (
            self.current_filter_category is None
            or self.current_filter_category == "Clear"
        ):
            return True
        else:
            return model[iter][1] == self.current_filter_category

    def on_selection_button_clicked(self, widget):
        """Called on any of the button clicks"""
        self.current_filter_category = widget.get_label()
        print('Debug: Action: ', '%s' % self.current_filter_category)
        if self.current_filter_category in 'Add':
            print("hmm")
        elif self.current_filter_category in 'Delete':
            print("hmm2")
        elif self.current_filter_category in 'Update':
            up.update()


win = TreeViewFilterWindow()
win.connect("destroy", Gtk.main_quit)
win.show_all()
Gtk.main()