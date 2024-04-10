import Scaffold from '@layouts/Scaffold';
import {styled} from 'nativewind';
import React, {useRef, useState} from 'react';
import {ScrollView, StyleSheet, View} from 'react-native';
import Carousel, {Pagination} from 'react-native-snap-carousel';
import onBoardingData, {OnBoardingItemData} from '../state/OnBoardingData';
import {useNavigation} from '@react-navigation/native';
import {NativeStackNavigationProp} from '@react-navigation/native-stack';
import {RootStackParamList} from '@constants/routes';
import {WIDTH} from '@constants/dimensions';
import Colors from '@constants/colors';
import PrimaryButton from '@components/button/PrimaryButton';
import OnboardingDescription from '../components/OnboardingDescription';
import OnBoardingItem from '../components/OnBoardingItem';
import GeneralImage from '@components/image/GeneralImage';
import Images from '@constants/images';

const StyledScrollView = styled(ScrollView);
const StyledView = styled(View);

const OnboardingScreen = () => {
  const isCarousel = useRef<Carousel<OnBoardingItemData>>(null);
  const [page, setPage] = useState<number>(0);
  const navigation =
    useNavigation<NativeStackNavigationProp<RootStackParamList>>();

  return (
    <Scaffold background={Colors.white}>
      <StyledScrollView
        className="mt-20"
        style={{alignSelf: 'center'}}
        showsVerticalScrollIndicator={false}>
        <Carousel
          ref={isCarousel}
          data={onBoardingData}
          onSnapToItem={item => setPage(item)}
          renderItem={({item}) => <OnBoardingItem key={item.id} item={item} />}
          sliderWidth={WIDTH}
          itemWidth={WIDTH}
          useScrollView={true}
          autoplay={true}
          loop={true}
        />
        <Pagination
          activeDotIndex={page}
          carouselRef={isCarousel as any}
          animatedTension={0}
          animatedDuration={0}
          tappableDots={true}
          inactiveDotOpacity={1}
          inactiveDotScale={0.4}
          dotsLength={onBoardingData.length}
          dotStyle={styles.dotStyle}
          inactiveDotStyle={styles.inactiveDotStyle}
        />
        <OnboardingDescription />
        <StyledView className="px-10">
          <PrimaryButton
            child={
              <GeneralImage
                url={Images.nfid}
                size={32}
                tintColor={Colors.white}
                classStyle="self-center"
              />
            }
            onPress={() => navigation.navigate('Login')}
            classStyle="my-8"
          />
        </StyledView>
      </StyledScrollView>
    </Scaffold>
  );
};

const styles = StyleSheet.create({
  dotStyle: {
    width: 30,
    borderRadius: 20,
    backgroundColor: Colors.primary_normal,
  },
  inactiveDotStyle: {
    width: 20,
    height: 20,
    backgroundColor: Colors.gray_light,
  },
});

export default OnboardingScreen;
