# VirtSynth

A bare bones virtual synthesizer.

## How does it work?

VirtSynth uses [additive synthesis](https://en.wikipedia.org/wiki/Additive_synthesis) to add multiple sinusoidal waves together to create the sound.
Following is the math behind it (might contain errors).

The frequency of a given key $n$ (tuned to 440 Hz) can be calculated with:

$$
freq(n) = 2^\frac{n - 49}{12} \times 440 \text{Hz}
$$

The amplitude of a sample can be expressed as:

$$
sample = G \times \sum\limits_{i = 0}^{|A|} \sin( A_i \times 2 \pi \times F_i + \phi_i) \times \frac{1.0}{max(1.0, \sum_{a \in A} a)}
$$

where:

* $G$ is the gain
* $A$ is a set of amplitudes greater than zero
* $F$ is a set of frequencies for the different keys
* $\phi$ is the phase of a frequency
* $\frac{1.0}{max(1.0, \sum_{a \in A} a)}$ is crucial in normalizing the amplitude to avoid clipping
