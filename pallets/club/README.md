# Club Member Management Pallet

This pallet is designed to facilitate club member management within a Substrate-based blockchain system. It introduces two roles, the root and the club owner, who have specific permissions and responsibilities within the club. The root role is responsible for creating new clubs, while the club owner role can add new members to the club and manage club-related settings.

## Roles

1. **Root**: The root role is responsible for creating new clubs. It has the following permissions:
   - Create a new club
   - Specify the initial club owner

2. **Club Owner**: The club owner role is assigned to a member of the club. It has the following permissions:
   - Add new members to the club
   - Transfer the club ownership to another member
   - Set the annual expense for club membership

## Club Creation and Ownership Transfer

To create a new club, the root role needs to pay a certain amount of tokens. The root role can also specify the initial club owner during club creation.

The club owner can transfer the ownership of the club to another member. This transfer allows the new club owner to exercise the club owner role's permissions, such as adding members and setting annual expenses.

## Membership and Renewal

To become a member of the club, an account needs to pay a certain amount of tokens as per the annual expenses set by the club owner. The maximum membership duration is limited to 100 years.

Membership has an expiration date, and members need to renew their membership periodically to maintain their active status within the club. If membership is not renewed within the specified timeframe, the member will no longer be considered an active member.

## License

This pallet is open-source software released under the [MIT License](LICENSE). Feel free to modify and distribute it as per your project's requirements.
