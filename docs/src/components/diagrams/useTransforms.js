import { useEdgesState, useNodesState, useReactFlow } from 'reactflow'
import * as whimsical from '@/components/diagrams/DiagramWhimsical'

const hide = (els) =>
  els.map((x) => ({ ...x, style: { ...x.style, opacity: 0 } }))

export function useTransforms(initialNodes, initialEdges, type) {
  const { fitView, getNode } = useReactFlow()
  const [nodes, setNodes] = useNodesState(hide(initialNodes))
  const [edges, setEdges] = useEdgesState(hide(initialEdges))

  const onInit = () => {
    setTimeout(() => {
      let newNodes = initialNodes
      let newEdges = initialEdges

      switch (type) {
        case 'whimsical':
          newNodes = whimsical.transformNodes(newNodes, getNode)
          newEdges = whimsical.transformEdges(newEdges)
          break
      }

      setNodes(newNodes)
      setEdges(newEdges)
      setTimeout(fitView, 20)
    }, 20)
  }

  return [nodes, edges, onInit]
}
