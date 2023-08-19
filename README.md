# paperdoll

[![Latest version](https://img.shields.io/crates/v/paperdoll.svg)](https://crates.io/crates/paperdoll)
[![Documentation](https://docs.rs/paperdoll/badge.svg)](https://docs.rs/paperdoll)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

2D pixel-based stationary paper doll model.

- It's 2D.
- It's pixel-based. Vector images and basic shapes are not supported in the current version.
- It's stationary. Animations and transformations are not supported in the current version.

Latest version: 1.

## Design

The model consists of three parts: **doll**, **slot**, and **fragment**.

### Doll

Dolls are the fundamental parts of your model. Normally, they represent faces, bodies, or any other assembled objects in your projects. A doll contains multiple slots.

### Slot

Slots are where your paper doll can have alternative styles. For example, in a doll that represents a human's face, they could be eyes, mouth, nose, and so on.

A slot can be placed in different positions inside the doll (eg. slot of eyes). Not all slots need to have images to be shown, they can be empty. For instance, an empty 'hair' slot means that the person is bald.

Each slot has several alternative images to display. they're called '**candidates**'. And those candidates are all defined as fragments.

### Fragment

Fragments are image assets that you can put into a slot as candidates. In `paperdoll`, all fragments are raster images. One fragment can be used in multiple slots.

There are two ways slots and their fragment candidates are connected.

- **Constrainted**. The fragment acts like the background of the slot. It will fill the whole space of the slot and resizes if needed.

- **Non-constrainted**. Slots and fragments are connected like mortises and tenons. There is an anchor point inside a slot. When a fragment is placed into a slot, the pivot point of that fragment will be placed in the same position as the anchor point. The fragment remains its original size and resizing will never happen.

![core-concept](https://raw.githubusercontent.com/fralonra/paperdoll/master/doc/paperdoll-concept.png)

## Container format

### ppd

`ppd` is a tar archive container for `paperdoll`. [Read more](https://github.com/fralonra/paperdoll-tar).

## Integrations

- [bevy-paperdoll](https://github.com/fralonra/bevy-paperdoll) for [Bevy](https://github.com/bevyengine/bevy).

## Tools

- [PpdEditor](https://github.com/fralonra/ppd-editor)

Editor for paperdoll.

![ppd-editor](https://raw.githubusercontent.com/fralonra/ppd-editor/master/resources/docs/ppd-editor.png)

- [PpdViewer](https://github.com/fralonra/ppd-editor)

Viewer for paperdoll.

![ppd-viewer](https://raw.githubusercontent.com/fralonra/ppd-editor/master/resources/docs/ppd-viewer.png)
