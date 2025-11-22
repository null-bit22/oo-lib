from .catalog import Catalog

class Member: 
    def __init__(self, name: str) -> None:
        self._name = name
        self._borrowed_ids = [] # list for borrowed Ids
        self._MAX_BORROW_LIMIT = 5

    @property
    def name(self) -> str:
        return self._name

    def borrow(self, id: str) -> None:
        # Check for max borrow limit
        if len(self._borrowed_ids) >= self._MAX_BORROW_LIMIT:
            raise ValueError("Maximum borrowed items reached!")

        # Check for duplicate borrows
        if id in self._borrowed_ids:
            raise ValueError(f"Duplicate borrow for id: {id}")
        self._borrowed_ids.append(id)

    def borrowed_ids(self) -> list:
        return self._borrowed_ids

    def return_item(self, id: str) -> None:
        self._borrowed_ids.remove(id)

    def list_details(self, cat: Catalog) -> list:
        lines = [] 

        for id in self._borrowed_ids:
            item = cat.get(id)
            lines.append(f"{item._title} ({item.days_allowed()} days)")

        return lines
    
    def is_borrowed_list_valid(self, cat: Catalog) -> bool:
        for id in self._borrowed_ids:
            if cat.get(id) == None:
                return False
        return True
