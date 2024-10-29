import { useEMRStore } from "@/store/patient-emr";
import { Button, Flex, Icon, Input, InputGroup, InputLeftElement, InputRightElement } from "@chakra-ui/react";
import { FaCalendarDay } from "react-icons/fa";
import { IoIosCloseCircle } from "react-icons/io";
import { IoSearch } from "react-icons/io5";

export default function EMRFilter() {
  const search = useEMRStore(state => state.search);
  const searchEMR = useEMRStore(state => state.searchEMR);

  const onFillSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
    searchEMR(e.target.value);
  }

  const onClearSearch = () => {
    searchEMR('');
  }

  return (
    <InputGroup>
      <InputLeftElement>
        <Icon as={IoSearch} boxSize={4} color={"neutral.500"} />
      </InputLeftElement>
      <Input
        placeholder="Search EMR"
        bg={"white"}
        value={search}
        onChange={onFillSearch}
        rounded={"xl"}
        border={"none"}
        outline={"none"}
        fontSize={'sm'}
        _placeholder={{ color: "neutral.500" }}
        color={"neutral.500"}
      />
      {search.length > 0 && (
        <InputRightElement onClick={onClearSearch} cursor={'pointer'}>
          <Icon as={IoIosCloseCircle} boxSize={4} color={"neutral.500"} />
        </InputRightElement>
      )}
    </InputGroup>
  )
}