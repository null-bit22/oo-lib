from .items import Item, Id

class Catalog: 
    def __init__(self) -> None:
        self._catalog = {} # start with empty dict of items

    def add(self, item: Item) -> None:
        key = item._id.value

        # Check for duplicate Ids
        if key in self._catalog:
            raise ValueError(f"Duplicate entry for id: {key}")
        self._catalog[item._id.value] = item

    def get(self, id: str) -> Item:
        return self._catalog.get(id)
