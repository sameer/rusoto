// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/rusoto/rusoto/master/assets/logo-square.png"
)]
//! <p><fullname>AWS Auto Scaling</fullname> <p>Use AWS Auto Scaling to create scaling plans for your applications to automatically scale your scalable AWS resources. </p> <p> <b>API Summary</b> </p> <p>You can use the AWS Auto Scaling service API to accomplish the following tasks:</p> <ul> <li> <p>Create and manage scaling plans</p> </li> <li> <p>Define target tracking scaling policies to dynamically scale your resources based on utilization</p> </li> <li> <p>Scale Amazon EC2 Auto Scaling groups using predictive scaling and dynamic scaling to scale your Amazon EC2 capacity faster</p> </li> <li> <p>Set minimum and maximum capacity limits</p> </li> <li> <p>Retrieve information on existing scaling plans</p> </li> <li> <p>Access current forecast data and historical forecast data for up to 56 days previous</p> </li> </ul> <p>To learn more about AWS Auto Scaling, including information about granting IAM users required permissions for AWS Auto Scaling actions, see the <a href="https://docs.aws.amazon.com/autoscaling/plans/userguide/what-is-aws-auto-scaling.html">AWS Auto Scaling User Guide</a>. </p></p>
//!
//! If you're using the service, you're probably looking for [AutoscalingPlansClient](struct.AutoscalingPlansClient.html) and [AutoscalingPlans](trait.AutoscalingPlans.html).

mod custom;
mod generated;
pub use custom::*;
pub use generated::*;
