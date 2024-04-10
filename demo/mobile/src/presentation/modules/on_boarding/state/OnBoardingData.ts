import Images from '@constants/images';
import Strings from '@constants/strings';
export interface OnBoardingItemData {
  id: number;
  title: string;
  body: string;
  icon: string; // Assuming icon is an image source, you can use 'any' or specify the correct type
}

const onBoardingData: OnBoardingItemData[] = [
  {
    id: 1,
    title: Strings.onBoardingItems.ob1Title,
    body: Strings.onBoardingItems.ob1Body,
    icon: Images.onBoard1,
  },
  {
    id: 2,
    title: Strings.onBoardingItems.ob2Title,
    body: Strings.onBoardingItems.ob2Body,
    icon: Images.onBoard2,
  },
  {
    id: 3,
    title: Strings.onBoardingItems.ob3Title,
    body: Strings.onBoardingItems.ob3Body,
    icon: Images.onBoard3,
  },
];

export default onBoardingData;
