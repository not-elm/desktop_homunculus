# How to Create VRMA

VRMA is an animation file format used for retargeting to VRM.
It is a relatively new format, and currently there are few tools that support it.

As far as I know, Blender's [VRM add-on for Blender](https://vrm-addon-for-blender.info/en/) is recommended because it
also allows configuration of facial expressions.

The application's built-in animations are also created using this
add-on, but since the T-Pose of animations exported from Blender differs from that of other exporters, there is a high
risk that animation interpolation may break. Therefore, I recommend exporting from this add-on whenever possible. For
more information on why differing T-poses can break animation interpolation,
see [here](https://github.com/not-elm/bevy_vrm1/issues/32).
