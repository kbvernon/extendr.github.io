-- Convert ![alt](src){#id .class key=val} into a raw <img> tag,
-- preserving id, classes, attributes, alt, and title.

local function html_escape(s)
  return (s:gsub("&", "&amp;")
    :gsub("<", "&lt;")
    :gsub(">", "&gt;")
    :gsub('"', "&quot;"))
end

function Image(img)
  local parts = pandoc.List()
  parts:insert('<img src="' .. html_escape(img.src) .. '"')

  if img.identifier and img.identifier ~= "" then
    parts:insert(' id="' .. html_escape(img.identifier) .. '"')
  end

  if #img.classes > 0 then
    parts:insert(' class="' .. html_escape(table.concat(img.classes, " ")) .. '"')
  end

  for k, v in pairs(img.attributes) do
    parts:insert(' ' .. k .. '="' .. html_escape(v) .. '"')
  end

  local alt = pandoc.utils.stringify(img.caption)
  if alt ~= "" then
    parts:insert(' alt="' .. html_escape(alt) .. '"')
  end

  if img.title and img.title ~= "" then
    parts:insert(' title="' .. html_escape(img.title) .. '"')
  end

  parts:insert('/>')
  return pandoc.RawInline("html", table.concat(parts))
end

-- Strip Pandoc's auto Figure wrapper so the <img> isn't enclosed in <figure>.
function Figure(fig)
  return pandoc.Plain(pandoc.utils.blocks_to_inlines(fig.content))
end

return {
  Figure = Figure,
  Image = Image,
}
