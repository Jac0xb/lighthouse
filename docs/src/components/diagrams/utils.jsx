export function LabelOrContent({ data }) {
  if (data.label) return <>{data.label}</>
  return (
    <>
      {data.content.map((child, index) => ({
        ...child,
        key: index.toString(36),
      }))}
    </>
  )
}

export function hasLabelOrContent(data) {
  return data.content?.length > 0 || data.label
}
