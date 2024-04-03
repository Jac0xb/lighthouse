import { getRectOfNodes, getTransformForBounds, useReactFlow } from 'reactflow'
import { toPng } from 'html-to-image'

export function useDownloadImage(
  ref,
  filename = 'diagram.png',
  imageWidth = 1024,
  imageHeight = 768
) {
  const { getNodes } = useReactFlow()

  const downloadDataUrl = (dataUrl) => {
    const a = document.createElement('a')
    a.setAttribute('download', filename)
    a.setAttribute('href', dataUrl)
    a.click()
  }

  const downloadImage = () => {
    if (!ref.current) return
    const viewport = ref.current.querySelector('.react-flow__viewport')
    const nodesBounds = getRectOfNodes(getNodes())
    const transform = getTransformForBounds(
      nodesBounds,
      imageWidth,
      imageHeight,
      0.5,
      2
    )
    const isDarkMode = document.querySelector('html').classList.contains('dark')
    toPng(viewport, {
      backgroundColor: isDarkMode ? '#1e293b' : '#f8fafc',
      width: imageWidth,
      height: imageHeight,
      style: {
        width: imageWidth,
        height: imageHeight,
        transform: `translate(${transform[0]}px, ${transform[1]}px) scale(${transform[2]})`,
      },
    }).then(downloadDataUrl)
  }

  return [downloadImage]
}
