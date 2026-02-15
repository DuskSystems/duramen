permit (principal: p, action: a, resource: r)
when { w }
unless { u }
advice { "doit" };

permit (principal in Group::"jane_friends",  // Policy c1
action in [PhotoOp::"view", PhotoOp::"comment"],
resource in Album::"jane_trips");

forbid (principal, action, resource)           // Policy c2
when   { "private" in resource.tags }  // resource.tags is a set of strings
unless { resource in user.account };
