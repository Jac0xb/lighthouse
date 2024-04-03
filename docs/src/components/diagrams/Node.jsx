import { Tag } from '@markdoc/markdoc'

export function transformNodeTag(node, config) {
  const attributes = node.transformAttributes(config)
  const allChildren = node.transformChildren(config)
  const children = allChildren.filter((child) => child.name !== 'Node')
  const treeNodes = allChildren.filter((child) => child.name === 'Node')
  return new Tag(this.render, { ...attributes, children: treeNodes }, children)
}
