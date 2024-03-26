import { useCallback } from 'react'
import {
  BaseEdge,
  EdgeLabelRenderer,
  Position,
  getBezierPath,
  getStraightPath,
  getSmoothStepPath,
  useStore,
  useReactFlow,
} from 'reactflow'
import { LabelOrContent, hasLabelOrContent } from './utils'

export function FloatingEdge({
  id,
  source,
  target,
  markerStart,
  markerEnd,
  style,
  data,
}) {
  const { getIntersectingNodes } = useReactFlow()
  const sourceNode = useStore(
    useCallback((store) => store.nodeInternals.get(source), [source])
  )
  const targetNode = useStore(
    useCallback((store) => store.nodeInternals.get(target), [target])
  )

  if (!sourceNode || !targetNode) {
    return null
  }

  const hasLabel = hasLabelOrContent(data)

  const candidates = getAllFromToPositions()
    .filter(([fromPosition, toPosition]) => {
      if (
        getAllPositions().includes(data.fromPosition) &&
        data.fromPosition !== fromPosition
      ) {
        return false
      }
      if (
        getAllPositions().includes(data.toPosition) &&
        data.toPosition !== toPosition
      ) {
        return false
      }
      return true
    })
    .map(([fromPosition, toPosition]) => {
      const fromAnchor = getAnchorFromPosition(sourceNode, fromPosition)
      const toAnchor = getAnchorFromPosition(targetNode, toPosition)
      return getCandidate(
        getIntersectingNodes,
        sourceNode,
        targetNode,
        fromAnchor,
        toAnchor,
        fromPosition,
        toPosition
      )
    })

  const bestCandidate = getBestCandidate(candidates)
  const [edgePath, labelX, labelY] = getCandidatePath(bestCandidate, data)

  return (
    <>
      <g className="stroke-red-500">
        <path
          id={id}
          style={style}
          d={edgePath}
          fill="none"
          className="react-flow__edge-path"
          markerEnd={markerEnd}
          markerStart={markerStart}
        />
      </g>
      {hasLabel && (
        <EdgeLabelRenderer>
          <div
            style={{
              position: 'absolute',
              transform: `translate(-50%, -50%) translate(${
                labelX + data.labelX
              }px,${labelY + data.labelY}px)`,
            }}
            className="rounded-md bg-slate-50 p-1 text-center text-xs text-slate-600 dark:bg-slate-800 dark:text-slate-300"
          >
            <LabelOrContent data={data} />
          </div>
        </EdgeLabelRenderer>
      )}
    </>
  )
}

/**
 * Create a candidate object from a pair of positions.
 */
function getCandidate(
  getIntersectingNodes,
  fromNode,
  toNode,
  fromAnchor,
  toAnchor,
  fromPosition = undefined,
  toPosition = undefined
) {
  fromPosition ??= getPositionFromAnchor(fromNode, fromAnchor)
  toPosition ??= getPositionFromAnchor(toNode, toAnchor)
  const distance = getDistance(fromAnchor, toAnchor)
  const intersectsWithNodes =
    getIntersectingNodes(getRectangle(fromAnchor, toAnchor)).length > 0
  const score = distance + (intersectsWithNodes ? 200 : 0)
  return {
    fromPosition,
    toPosition,
    fromAnchor,
    toAnchor,
    distance,
    intersectsWithNodes,
    score,
  }
}

/**
 * Get the best scoring candidate edge.
 */
function getBestCandidate(candidates) {
  return candidates.reduce((best, candidate) => {
    return candidate.score < best.score ? candidate : best
  }, candidates[0])
}

/**
 * Get the path and label coordinates for a candidate edge.
 */
function getCandidatePath(candidate, data) {
  const params = {
    sourceX: candidate.fromAnchor.x,
    sourceY: candidate.fromAnchor.y,
    targetX: candidate.toAnchor.x,
    targetY: candidate.toAnchor.y,
    sourcePosition: candidate.fromPosition,
    targetPosition: candidate.toPosition,
    borderRadius: 5,
  }
  if (data.path === 'bezier') return getBezierPath(params)
  if (data.path === 'straight') return getStraightPath(params)
  return getSmoothStepPath(params)
}

/**
 * Returns the anchor point on the border of a node
 * such that it belongs to the line that goes from the center of the node
 * to the center of another target node.
 * @see https://math.stackexchange.com/questions/1724792/an-algorithm-for-finding-the-intersection-point-between-a-center-of-vision-and-a
 */
function getAnchorFromNodeIntersection(node, targetNode) {
  const {
    width: nodeWidth,
    height: nodeHeight,
    positionAbsolute: nodePosition,
  } = node
  const targetPosition = targetNode.positionAbsolute

  const w = nodeWidth / 2
  const h = nodeHeight / 2

  const x2 = nodePosition.x + w
  const y2 = nodePosition.y + h
  const x1 = targetPosition.x + w //(targetNode.width / 2)
  const y1 = targetPosition.y + h //(targetNode.height / 2)

  const xx1 = (x1 - x2) / (2 * w) - (y1 - y2) / (2 * h)
  const yy1 = (x1 - x2) / (2 * w) + (y1 - y2) / (2 * h)
  const a = 1 / (Math.abs(xx1) + Math.abs(yy1))
  const xx3 = a * xx1
  const yy3 = a * yy1
  const x = w * (xx3 + yy3) + x2
  const y = h * (-xx3 + yy3) + y2

  return { x, y }
}

/**
 * Returns the absolute coordinates of a point on the border of a node
 * at the given position (top, right, bottom or right).
 */
function getAnchorFromPosition(node, position) {
  const nx = node.positionAbsolute.x
  const ny = node.positionAbsolute.y
  const w = node.width
  const h = node.height
  if (position === Position.Top) return { x: nx + w / 2, y: ny }
  if (position === Position.Right) return { x: nx + w, y: ny + h / 2 }
  if (position === Position.Bottom) return { x: nx + w / 2, y: ny + h }
  if (position === Position.Left) return { x: nx, y: ny + h / 2 }
  return { x: nx + w / 2, y: ny }
}

/**
 * Returns the position (top, right, bottom or right)
 * of a point on the border of a node, i.e. an anchor point.
 */
function getPositionFromAnchor(node, anchor) {
  const n = { ...node.positionAbsolute, ...node }
  const nx = Math.round(n.x)
  const ny = Math.round(n.y)
  const ax = Math.round(anchor.x)
  const ay = Math.round(anchor.y)

  if (ax <= nx + 1) return Position.Left
  if (ax >= nx + n.width - 1) return Position.Right
  if (ay <= ny + 1) return Position.Top
  if (ay >= n.y + n.height - 1) return Position.Bottom
  return Position.Top
}

/**
 * Returns all possible positions (top, right, bottom or right).
 */
function getAllPositions() {
  return [Position.Top, Position.Right, Position.Bottom, Position.Left]
}

/**
 * Returns all possible combinations of positions for two nodes.
 */
function getAllFromToPositions() {
  return getAllPositions().flatMap((from) =>
    getAllPositions().map((to) => [from, to])
  )
}

/**
 * Returns the euclidean distance between two points.
 */
function getDistance(from, to) {
  return Math.sqrt(Math.pow(from.x - to.x, 2) + Math.pow(from.y - to.y, 2))
}

/**
 * Returns a rectangle defined by two points.
 */
function getRectangle(from, to) {
  return {
    x: Math.min(from.x, to.x),
    y: Math.min(from.y, to.y),
    width: Math.abs(from.x - to.x),
    height: Math.abs(from.y - to.y),
  }
}
