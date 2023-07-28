#include "cosave.h"

#include "lib.rs.h"

inline const auto CYCLE_RECORD = _byteswap_ulong('CYCL');

namespace cosave
{

	void gameSavedHandler(SKSE::SerializationInterface* cosave)
	{
		if (!cosave->OpenRecord(CYCLE_RECORD, 0))
		{
			logger::error("Unable to open record to write cosave data.");
			return;
		}

		// The format is an ad-hoc bag of bytes that we interpret
		// as we wish. What I'm going to do is serialize on the Rust side
		// to a bag of bytes and then just write the bag.

		rust::Vec<uint8_t> buffer = serialize_cycles();
		cosave->WriteRecordData(buffer.size());
		rust::Vec<uint8_t>::iterator iter;
		for (iter = buffer.begin(); iter != buffer.end(); ++iter) { cosave->WriteRecordData(iter); }
	}

	void revertHandler(SKSE::SerializationInterface* cosave)
	{
		// TODO reset
	}

	void gameLoadedHandler(SKSE::SerializationInterface* cosave)
	{
		std::uint32_t type;
		std::uint32_t version;
		std::uint32_t size;

		while (cosave->GetNextRecordInfo(type, version, size))
		{
			if (type == CYCLE_RECORD)
			{
				int bufferSize;
				rust::Vec<uint8_t> buffer;
				cosave->ReadRecordData(bufferSize);
				for (; bufferSize > 0; --bufferSize)
				{
					// this feels staggeringly inefficient, but first I gotta make it work
					uint8_t next;
					cosave->ReadRecordData(next);
					buffer.push_back(next);
				}
				cycle_loaded_from_cosave(buffer);
			}
			else { logger::warn("Unknown record type in cosave; type={}", type); }
		}
	}

}
