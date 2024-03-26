import { Tag } from '@markdoc/markdoc'
import clsx from 'clsx'
import ReactFlow, { Background, Panel, ReactFlowProvider } from 'reactflow'
import { useRef } from 'react'

import { Icon } from '@/components/icons'
import * as whimsical from './DiagramWhimsical'
import { FloatingEdge } from './FloatingEdge'
import { useFullscreen } from '@/components/diagrams/useFullscreen'
import { useDownloadImage } from '@/components/diagrams/useDownloadImage'
import { useTransforms } from '@/components/diagrams/useTransforms'

const nodeTypes = {
  whimsical: whimsical.Node,
}

const edgeTypes = {
  floating: FloatingEdge,
}

export function Diagram(props) {
  return (
    <ReactFlowProvider>
      <WrappedDiagram {...props} />
    </ReactFlowProvider>
  )
}

export function WrappedDiagram({
  height = 'h-64 md:h-96',
  nodes: initialNodes,
  edges: initialEdges,
  type,
}) {
  const ref = useRef(null)
  const [fullscreen, toggleFullscreen] = useFullscreen()
  const [downloadImage] = useDownloadImage(ref)
  const [nodes, edges, onInit] = useTransforms(initialNodes, initialEdges, type)

  return (
    <div
      className={clsx('diagram-prose w-full', {
        [height]: !fullscreen,
        relative: !fullscreen,
        'fixed inset-0 z-[99999] h-full w-full': fullscreen,
      })}
    >
      <ReactFlow
        ref={ref}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        nodes={nodes}
        edges={edges}
        onInit={onInit}
        fitView
        nodesDraggable={false}
        nodesConnectable={false}
        nodesFocusable={false}
        edgesFocusable={false}
        edgesUpdatable={false}
        className="rounded-xl bg-slate-50 dark:bg-slate-800"
      >
        <Background
          color="currentColor"
          className="text-slate-400 dark:text-slate-600"
        />
      </ReactFlow>
      <Panel position="top-right">
        <div className="flex gap-1 rounded-md bg-white p-1 shadow-md shadow-black/5 ring-1 ring-black/5 dark:bg-slate-900 dark:ring-white/10">
          <button
            onClick={downloadImage}
            className="rounded p-1 hover:bg-slate-100 hover:dark:bg-slate-600/40"
          >
            <Icon icon="InboxArrowDown" className="h-5 w-5" />
          </button>
          <button
            onClick={toggleFullscreen}
            className="rounded p-1 hover:bg-slate-100 hover:dark:bg-slate-600/40"
          >
            <Icon
              icon={fullscreen ? 'ArrowsPointingIn' : 'ArrowsPointingOut'}
              className="h-5 w-5"
            />
          </button>
        </div>
      </Panel>
    </div>
  )
}

export function transformDiagramTag(node, config) {
  const attributes = node.transformAttributes(config)
  const children = node.transformChildren(config)

  const type = attributes.type ?? 'whimsical'
  const markdocNodes = children.filter((child) => child.name === 'Node')
  const markdocEdges = children.filter((child) => child.name === 'Edge')
  const nodes = getNodesFromMarkdoc(markdocNodes, type)
  const edges = getEdgesFromMarkdoc(markdocEdges, type)

  return new Tag(this.render, { ...attributes, nodes, edges, type }, children)
}

function getNodesFromMarkdoc(markdocNodes, type, treeParent = undefined) {
  return markdocNodes.flatMap(({ attributes, children: content }, index) => {
    const id = attributes.id ?? `${treeParent ?? 'node'}-${index}`
    const treeChildren =
      attributes.children.length > 0
        ? getNodesFromMarkdoc(attributes.children, type, id)
        : []
    if (!treeParent) {
      treeChildren.forEach((child) => {
        child.data.tree.ancestor = id
      })
    }
    const directTreeChildren = treeChildren.filter(
      (child) => child.data.tree.parent === id
    )
    const directTreeChildrenIds = directTreeChildren.map((child) => child.id)
    directTreeChildren.forEach((child) => {
      child.data.tree.siblings = directTreeChildrenIds
    })
    const node = {
      id,
      type: attributes.type ?? (type in nodeTypes ? type : 'whimsical'),
      parentNode: treeParent ?? attributes.parent,
      data: {
        content,
        label: attributes.label,
        theme: attributes.theme,
        sections: attributes.sections,
        tree: {
          ancestor: null,
          parent: treeParent ?? null,
          parentIndex: index,
          children: directTreeChildrenIds,
          siblings: [],
        },
      },
      position: {
        x: parseInt(attributes.x ?? 0, 10),
        y: parseInt(attributes.y ?? 0, 10),
      },
      style: {
        zIndex: attributes.z,
      },
    }

    return [node, ...treeChildren]
  })
}

function getEdgesFromMarkdoc(markdocEdges, type) {
  return markdocEdges.map(({ attributes, children: content }, index) => ({
    id: attributes.id ?? `edge-${index}`,
    type: attributes.type ?? (type in edgeTypes ? type : 'floating'),
    source: attributes.from,
    target: attributes.to,
    animated: attributes.animated ?? false,
    data: {
      content,
      label: attributes.label,
      labelX: attributes.labelX ?? 0,
      labelY: attributes.labelY ?? 0,
      dashed: attributes.dashed,
      arrow: attributes.arrow,
      theme: attributes.theme,
      fromPosition: attributes.fromPosition,
      toPosition: attributes.toPosition,
      path: attributes.path,
    },
  }))
}
