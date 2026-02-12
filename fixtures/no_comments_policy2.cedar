permit (
    principal == IAM::Principal::"arn:aws:iam::12345678901:user/Dave",
    action == S3::Action::"GetAccountPublicAccessBlock",
    resource == Account::"12345678901"
);
