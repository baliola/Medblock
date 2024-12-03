"use client";

import { Flex, Image, Text } from '@chakra-ui/react';
import { A11y, Pagination } from 'swiper/modules';
import { Swiper, SwiperSlide } from 'swiper/react';
import { guideSlider } from '@/constants/contents/auth/guide/slider';

import 'swiper/css';
import 'swiper/css/pagination';
import './_header.css';

export default function AuthGuideHeader() {
  return (
    <Flex w={"full"} h={"fit-content"}>
      <Swiper
        modules={[Pagination, A11y]}
        slidesPerView={1}
        pagination={{
          clickable: true,
          el: ".custom-pagination",
          bulletClass: "custom-bullet",
          bulletActiveClass: "custom-bullet-active",
          bulletElement: "button",
        }}
        autoplay={{ delay: 3000 }}
        className='custom-swiper'
        loop={true}
      >
        {guideSlider.map((data, index) => (
          <SwiperSlide key={index} style={{ height: "fit-content" }}>
            <Flex
              direction={"column"}
              w={"full"}
              align={"center"}
              justify={'center'}
              px={5}
              gap={5}
            >
              <Text fontSize={'xl'} fontWeight={'semibold'} textAlign={'center'} color={"black"}>
                {data.title}
              </Text>
              <Image src={data.image} alt={data.title} w={28} />
              <Text fontSize={'sm'} textAlign={"center"} lineHeight={1.8}>
                {data.description}
              </Text>
            </Flex>
          </SwiperSlide>
        ))}
      </Swiper>
    </Flex>
  );
}
