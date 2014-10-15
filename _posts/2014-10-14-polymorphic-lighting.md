---
title: Polymorphic Lighting
commit: f75ad820e36cf8748da20bc423a1bc45a6197663
---



Today I decided that I would implement polymorphic lighting. Since I'd already added polymorphic geometry, it was actually quite simple. The process I needed to follow was as follows:

* Add an `Illuminator` trait that provides an `illuminate` function to replace the single-version one I was using.
* Replace the contents of `SceneLight` with a boxed `Illuminator`, which ends up looking like `Box<Illuminator+Send+Sync+'static>`.
* Add some new lights which `impl Illuminator`, I added `PointLight` and `DirectionalLight`.
* Replace the light parsing code to generate a boxed `DirectionalLight` (My scene format doesn't currently support other lights. Baby steps.)

It was mostly a smooth process. I had trouble at one point debugging the lights because `#[deriving (Show)]` apparently doesn't work for `Box<T>`, and so I had to try to create my own implementation of `Show`. I ended up working around it, so I can't give an explanation at this time, but I'll certainly look into it.
