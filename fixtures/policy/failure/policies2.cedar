permit (
    principal in Group::"jane_friends",  // Policy c1
    action in [PhotoOp::"view", PhotoOp::"comment"],
    resource in Album::"jane_trips",
    context:Group
);
