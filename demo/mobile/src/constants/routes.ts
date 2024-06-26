export default class AppRoutes {
  // Main
  static readonly main = {
    main: 'main',
    home: 'Home',
    file: 'File',
    history: 'History',
    setting: 'Setting',
  };
}

export type RootStackParamList = {
  // Onboarding
  Onboarding: undefined;

  // Auth
  Login: undefined;

  // Verification
  Unverified: undefined;
  FillPersonalInformation: undefined;
  Verified: undefined;

  // Main
  MainNavigation: undefined;

  // Home
  HomeNavigation: undefined;
  ConsentCode: undefined;
  EmrDetail: undefined;
  ProfileInformation: undefined;
  RevokeAccess: undefined;
  Notification: undefined;
};
