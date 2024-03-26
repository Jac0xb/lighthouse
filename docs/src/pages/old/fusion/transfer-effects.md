---
title: Transfer Effects
metaTitle: Fusion - Transfer Effects
description: Effects that can be triggered on compose and decompose events.
---

The Trifle program includes the ability to trigger special events on compose/decompose events. Depending on the choice of effects to trigger, this allows for powerful combinations and enhanced behavior to be created, depending on the creator's preferences. {% .lead %}

## Track

This effect tells the Trifle account to track the token that has been added by `transfer_in`. It is enabled by default.

## Burn

This tells the `transfer_in` function to burn the token rather than transfer it into the Creator Owned Escrow on the NFT. This can be used for consumable tokens such as in-game power-ups or food for a virtual pet.

## Freeze

The Freeze effect freezes the token in place on composition, not allowing the holder to transfer the token back out.

## Freeze Parent

One major caveat for legacy projects who want to add composability to their NFT projects is the fact that trait swapping can result in undesirable changes in rarity rankings or volatile price action based on traits. Should a project desire, they may implement the Freeze Parent effect which freezes the parent NFT when it has tokens in a slot with this effect enabled. This means no transfers or sales are possible while in a composed state. The NFT is automatically unfrozen when no slots with this effect have tokens in them.
