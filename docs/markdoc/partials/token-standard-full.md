{% table %}

- Field
- Type
- Description

---

- **name**
- string
- Name of the asset.

---

- **symbol**
- string
- Symbol of the asset.

---

- **description**
- string
- Description of the asset.

---

- **image**
- string
- URI pointing to the asset's logo.

---

- **animation_url**
- string
- URI pointing to the asset's animation.

---

- **external_url**
- string
- URI pointing to an external URL defining the asset — e.g. the game's main site.

---

- **attributes**
- array
- Array of attributes defining the characteristics of the asset.
  - **trait_type** (string): The type of attribute.
  - **value** (string): The value for that attribute.

---

- **properties**
- object
- Additional properties that define the asset.
  - **files** (array): Additional files to include with the asset.
    - **uri** (string): The file's URI.
    - **type** (string): The file's type. E.g. `image/png`, `video/mp4`, etc.
    - **cdn** (boolean, optional): Whether the file is served from a CDN.
  - **category** (string): A media category for the asset. E.g. `video`, `image`, etc.

{% /table %}
